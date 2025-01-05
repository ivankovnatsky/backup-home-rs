use anyhow::Result;
use std::process::Command;
use tracing::info;

pub async fn upload_to_rclone(source: &str, destination: &str) -> Result<()> {
    info!("Uploading backup to: {}", destination);

    let status = Command::new("rclone")
        .args(["copy", "--progress", source, destination])
        .status()?;

    if !status.success() {
        anyhow::bail!("rclone upload failed with status: {}", status);
    }

    info!("Upload completed successfully");
    Ok(())
}
