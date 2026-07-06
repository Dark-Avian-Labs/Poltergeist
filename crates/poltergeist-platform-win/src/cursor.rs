pub fn position() -> Option<(i32, i32)> {
    #[cfg(windows)]
    {
        crate::ffi::get_cursor_pos()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

pub fn primary_buttons_down() -> bool {
    #[cfg(windows)]
    {
        crate::ffi::primary_mouse_buttons_down()
    }
    #[cfg(not(windows))]
    {
        false
    }
}
