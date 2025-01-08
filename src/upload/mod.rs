use anyhow::Result;
use serde_json::json;
use tracing::info;
use std::time::Instant;

pub async fn upload_to_rclone(source: &str, destination: &str) -> Result<()> {
    info!("Uploading backup to: {}", destination);

    // Initialize librclone
    librclone::initialize();

    let start_time = Instant::now();

    // Create the copy request for a single file
    let request = json!({
        "srcFs": std::path::Path::new(source).parent().unwrap_or(std::path::Path::new("")).to_string_lossy(),
        "srcRemote": std::path::Path::new(source).file_name().unwrap_or_default().to_string_lossy(),
        "dstFs": destination,
        "dstRemote": std::path::Path::new(source).file_name().unwrap_or_default().to_string_lossy(),
    });

    // Execute the copy operation
    librclone::rpc("operations/copyfile", &request.to_string())
        .map_err(|e| anyhow::anyhow!("RPC error: {}", e))?;

    // Show final stats
    let elapsed = start_time.elapsed().as_secs_f64();
    let file_size = std::fs::metadata(source)?.len() as f64 / 1024.0 / 1024.0;
    let mb_per_sec = file_size / elapsed;

    info!(
        "Upload completed: {:.2} MB transferred ({:.2} MB/s)",
        file_size,
        mb_per_sec
    );

    librclone::finalize();
    Ok(())
}
