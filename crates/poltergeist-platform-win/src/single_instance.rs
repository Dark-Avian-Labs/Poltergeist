#[cfg(windows)]
use windows::Win32::Foundation::HANDLE;

#[cfg(windows)]
pub struct SingleInstanceGuard {
    handle: HANDLE,
}

#[cfg(windows)]
impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        crate::ffi::close_handle_best_effort(self.handle);
    }
}

#[cfg(windows)]
pub enum AcquireResult {
    Acquired(SingleInstanceGuard),
    AlreadyRunning,
}

#[cfg(windows)]
pub fn try_acquire(is_admin: bool) -> AcquireResult {
    let name = mutex_name(is_admin);
    let wide: Vec<u16> = name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    match crate::ffi::create_global_mutex(false, &wide) {
        crate::ffi::CreateGlobalMutexOutcome::Created(handle) => {
            AcquireResult::Acquired(SingleInstanceGuard { handle })
        }
        crate::ffi::CreateGlobalMutexOutcome::AlreadyRunning => AcquireResult::AlreadyRunning,
        crate::ffi::CreateGlobalMutexOutcome::CreateFailed => {
            AcquireResult::Acquired(SingleInstanceGuard {
                handle: HANDLE(std::ptr::null_mut()),
            })
        }
    }
}

#[cfg(windows)]
pub fn show_already_running_dialog(is_admin: bool) {
    let title = if is_admin {
        "Poltergeist [ADMIN]"
    } else {
        "Poltergeist"
    };
    let body = "Poltergeist is already running.\nLook for the icon in the system tray.";

    let title_w: Vec<u16> = title
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let body_w: Vec<u16> = body
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    crate::ffi::message_box_information(&title_w, &body_w);
}

#[cfg(windows)]
fn mutex_name(is_admin: bool) -> &'static str {
    if is_admin {
        "Global\\PoltergeistSnippetManager.Admin"
    } else {
        "Global\\PoltergeistSnippetManager"
    }
}

#[cfg(not(windows))]
pub struct SingleInstanceGuard;

#[cfg(not(windows))]
pub enum AcquireResult {
    Acquired(SingleInstanceGuard),
    AlreadyRunning,
}

#[cfg(not(windows))]
pub fn try_acquire(_is_admin: bool) -> AcquireResult {
    AcquireResult::Acquired(SingleInstanceGuard)
}

#[cfg(not(windows))]
pub fn show_already_running_dialog(_is_admin: bool) {}
