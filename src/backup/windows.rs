use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use tracing::info;

pub fn create_archive(source: &str, backup_path: &PathBuf, compression_level: u32) -> Result<()> {
    // Check if 7z is available
    if Command::new("7z").arg("--help").output().is_err() {
        anyhow::bail!("7z is not found in PATH. Please install 7-Zip first.");
    }

    // Build 7z command
    let mut command = Command::new("7z");
    command
        .arg("a") // Add to archive
        .arg("-tzip") // ZIP format
        .arg(format!("-mx={}", compression_level.clamp(0, 9))) // Compression level 0-9
        .arg("-r") // Recursive
        .arg("-y") // Yes to all queries
        .arg("-ssw") // Compress files open for writing
        .arg(backup_path) // Output file
        .arg(format!("{}\\*", source)); // Source directory with all files

    // Add exclude patterns
    for pattern in crate::platform::get_exclude_patterns() {
        command.arg(format!("-xr!\"{}\"", pattern));
    }

    let output = command.output()?;

    // Check exit code (0 = success, 1 = warning but still ok, 2 = warning with skips)
    match output.status.code() {
        Some(0) => info!("Archive created successfully with no warnings."),
        Some(1) => info!("Archive created successfully with some files skipped."),
        Some(2) => info!("Archive created with some files skipped (locked files or permissions)."),
        _ => anyhow::bail!("7-Zip failed with exit code: {:?}", output.status.code()),
    }

    // Verify the archive was created
    if !backup_path.exists() {
        anyhow::bail!("Failed to create backup archive");
    }

    Ok(())
}
