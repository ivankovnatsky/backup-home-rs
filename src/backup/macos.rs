use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn create_archive(source: &str, backup_path: &PathBuf, compression_level: u32) -> Result<()> {
    // Build the tar command with explicit path to macOS tar
    let mut command = Command::new("/usr/bin/tar");
    command
        .arg("--strip-components=2")
        .arg("-cvf")
        .arg("-");

    // Add exclude patterns before the source
    for pattern in crate::platform::get_exclude_patterns() {
        command.arg("--exclude");
        command.arg(&pattern);
    }

    // Add source last
    command.arg(source);

    // Debug: Print the command and its arguments
    println!("Executing command: /usr/bin/tar {}", command
        .get_args()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" "));

    let tar_child = command.stdout(std::process::Stdio::piped()).spawn()?;

    let mut pigz_process = Command::new("pigz")
        .arg("-c")
        .arg(format!("-{}", compression_level.clamp(1, 9)))
        .stdin(tar_child.stdout.unwrap())
        .stdout(std::fs::File::create(backup_path)?)
        .spawn()?;

    // Wait for the pipeline to complete
    let status = pigz_process.wait()?;
    if !status.success() {
        anyhow::bail!("Backup creation failed");
    }

    Ok(())
}
