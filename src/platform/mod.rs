use anyhow::Result;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub fn get_exclude_patterns() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        windows::get_windows_excludes()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_macos_excludes()
    }
    #[cfg(target_os = "linux")]
    {
        linux::get_linux_excludes()
    }
}

pub fn get_temp_dir() -> Result<std::path::PathBuf> {
    Ok(std::env::temp_dir())
}
