#[cfg(windows)]
pub fn system_uses_light_theme() -> Option<bool> {
    crate::ffi::query_apps_use_light_theme()
}

#[cfg(not(windows))]
pub fn system_uses_light_theme() -> Option<bool> {
    None
}
