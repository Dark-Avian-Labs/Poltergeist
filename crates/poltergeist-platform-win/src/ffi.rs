//! Centralized Windows FFI. Every `unsafe` block in this crate lives here so call sites
//! stay auditable in one place. Win32 entry points are `unsafe` in `windows-rs` because
//! Rust cannot prove pointer validity, layout, or OS preconditions — the wrappers document
//! the contracts we rely on.

use windows::Win32::Foundation::{HANDLE, HWND, POINT};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyState, MapVirtualKeyW, SendInput, VkKeyScanW, INPUT, MAPVK_VK_TO_VSC,
    VIRTUAL_KEY, VK_CAPITAL, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetForegroundWindow, SetForegroundWindow,
};

pub fn get_cursor_pos() -> Option<(i32, i32)> {
    // SAFETY: `POINT` is a stack value; `GetCursorPos` writes the cursor position or fails.
    unsafe {
        let mut p = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut p as *mut POINT).is_ok() {
            Some((p.x, p.y))
        } else {
            None
        }
    }
}

pub fn primary_mouse_buttons_down() -> bool {
    // SAFETY: `GetAsyncKeyState` is documented to accept any virtual-key code; we only pass
    // standard mouse button VK constants.
    unsafe {
        let any =
            |vk: VIRTUAL_KEY| -> bool { (GetAsyncKeyState(vk.0 as i32) as u16 & 0x8000) != 0 };
        any(VK_LBUTTON) || any(VK_RBUTTON) || any(VK_MBUTTON)
    }
}

pub fn get_foreground_window_hwnd() -> Option<isize> {
    // SAFETY: `GetForegroundWindow` takes no pointers; return value may be null.
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            None
        } else {
            Some(hwnd.0 as isize)
        }
    }
}

pub fn set_foreground_window(hwnd: isize) -> bool {
    // SAFETY: We treat `hwnd` as an opaque window handle from the same API family; invalid
    // handles fail gracefully per Win32.
    unsafe { SetForegroundWindow(HWND(hwnd as *mut core::ffi::c_void)).as_bool() }
}

pub fn send_keyboard_input(inputs: &[INPUT]) -> u32 {
    // SAFETY: `SendInput` requires a slice of correctly initialized `INPUT` structs and the
    // element size in bytes. Callers build `INPUT` values the same way as Win32 examples.
    unsafe { SendInput(inputs, std::mem::size_of::<INPUT>() as i32) }
}

pub fn caps_lock_toggled_on() -> bool {
    // SAFETY: `GetKeyState` accepts documented VK codes; low bit indicates toggle state for Caps Lock.
    unsafe { (GetKeyState(VK_CAPITAL.0 as i32) as i16 & 0x0001) != 0 }
}

pub fn vk_key_scan_w(ch: u16) -> i16 {
    // SAFETY: `VkKeyScanW` accepts any UTF-16 code unit; we pass a single BMP character unit.
    unsafe { VkKeyScanW(ch) }
}

pub fn map_virtual_key_vk_to_vsc(vk: u32) -> u32 {
    // SAFETY: `MapVirtualKeyW` is documented for VK values; unused here for side effects only.
    unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_VSC) }
}

pub fn query_apps_use_light_theme() -> Option<bool> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, KEY_READ, REG_DWORD,
        REG_VALUE_TYPE,
    };

    let subkey: Vec<u16> = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\0"
        .encode_utf16()
        .collect();
    let value_name: Vec<u16> = "AppsUseLightTheme\0".encode_utf16().collect();

    // SAFETY: `subkey` / `value_name` are NUL-terminated wide strings; registry APIs read only.
    // `data` is `u32` with size matching REG_DWORD; pointer passed only for the query call duration.
    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        let open_status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if open_status != ERROR_SUCCESS {
            return None;
        }

        let mut value_type = REG_VALUE_TYPE(0);
        let mut data: u32 = 0;
        let mut data_size: u32 = std::mem::size_of::<u32>() as u32;
        let query_status = RegQueryValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );
        let _ = RegCloseKey(hkey);

        if query_status != ERROR_SUCCESS || value_type != REG_DWORD {
            return None;
        }
        Some(data != 0)
    }
}

pub fn close_handle_best_effort(handle: HANDLE) {
    if handle.is_invalid() {
        return;
    }
    // SAFETY: We only close handles we own; invalid handles are skipped above.
    unsafe {
        let _ = windows::Win32::Foundation::CloseHandle(handle);
    }
}

pub enum CreateGlobalMutexOutcome {
    Created(HANDLE),
    AlreadyRunning,
    CreateFailed,
}

pub fn create_global_mutex(
    initial_owner: bool,
    name_utf16_nul: &[u16],
) -> CreateGlobalMutexOutcome {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;

    // SAFETY: `name_utf16_nul` must be NUL-terminated; callers build it with `.chain(once(0))`.
    unsafe {
        let handle = match CreateMutexW(None, initial_owner, PCWSTR(name_utf16_nul.as_ptr())) {
            Ok(h) => h,
            Err(_) => return CreateGlobalMutexOutcome::CreateFailed,
        };
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = windows::Win32::Foundation::CloseHandle(handle);
            return CreateGlobalMutexOutcome::AlreadyRunning;
        }
        CreateGlobalMutexOutcome::Created(handle)
    }
}

pub fn message_box_information(title_utf16_nul: &[u16], body_utf16_nul: &[u16]) {
    use windows::core::PCWSTR;
    use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    // SAFETY: Both slices are NUL-terminated wide strings with stable pointers for the call.
    unsafe {
        let _ = MessageBoxW(
            None,
            PCWSTR(body_utf16_nul.as_ptr()),
            PCWSTR(title_utf16_nul.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
