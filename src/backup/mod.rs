use anyhow::Result;
use std::path::Path;
use tracing::info;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

const DEFAULT_COMPRESSION_LEVEL: u32 = 6;

pub fn create_backup(source: &str, compression_level: Option<u32>) -> Result<String> {
    let source_path = Path::new(source);
    if !source_path.exists() {
        anyhow::bail!("Source directory does not exist: {}", source);
    }

    // Use provided compression level or default to 6
    let compression = compression_level
        .unwrap_or(DEFAULT_COMPRESSION_LEVEL)
        .clamp(0, 9);

    let temp_dir = crate::platform::get_temp_dir()?;
    let username = whoami::username();
    let backup_path = temp_dir.join(format!("{}.{}", username, get_archive_extension()));

    info!("Creating backup of: {}", source);
    info!("Backup file: {}", backup_path.display());
    info!("Using compression level: {}", compression);

    #[cfg(target_os = "macos")]
    macos::create_archive(source, &backup_path, compression)?;

    #[cfg(target_os = "windows")]
    windows::create_archive(source, &backup_path, compression)?;

    Ok(backup_path.to_string_lossy().to_string())
}

#[cfg(target_os = "windows")]
fn get_archive_extension() -> &'static str {
    "zip"
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_archive_extension() -> &'static str {
    "tar.gz"
}
