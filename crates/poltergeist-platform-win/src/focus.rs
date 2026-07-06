#[cfg(windows)]
pub type WindowHandle = isize;
#[cfg(not(windows))]
pub type WindowHandle = i64;

pub fn current_foreground() -> Option<WindowHandle> {
    #[cfg(windows)]
    {
        crate::ffi::get_foreground_window_hwnd()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

pub fn set_foreground(hwnd: WindowHandle) -> bool {
    #[cfg(windows)]
    {
        crate::ffi::set_foreground_window(hwnd)
    }
    #[cfg(not(windows))]
    {
        let _ = hwnd;
        false
    }
}
