use anyhow::Result;
use serde_json::json;
use tracing::info;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn upload_to_rclone(source: &str, destination: &str) -> Result<()> {
    info!("Uploading backup to: {}", destination);

    // Initialize librclone
    librclone::initialize();

    let start_time = Instant::now();
    let update_interval = Duration::from_secs(5);

    // Create the copy request for a single file
    let request = json!({
        "srcFs": std::path::Path::new(source).parent().unwrap_or(std::path::Path::new("")).to_string_lossy(),
        "srcRemote": std::path::Path::new(source).file_name().unwrap_or_default().to_string_lossy(),
        "dstFs": destination,
        "dstRemote": std::path::Path::new(source).file_name().unwrap_or_default().to_string_lossy(),
        "_async": false,
    });

    // Start the copy operation
    librclone::rpc("operations/copyfile", &request.to_string())
        .map_err(|e| anyhow::anyhow!("RPC error: {}", e))?;

    // Poll for stats until the job completes
    loop {
        sleep(update_interval).await;

        // Try both possible stats endpoints
        let stats_json = librclone::rpc("rc/stats", "{}")
            .or_else(|_| librclone::rpc("core/stats", "{}"))
            .map_err(|e| anyhow::anyhow!("Stats RPC error: {}", e))?;

        if let Ok(stats) = serde_json::from_str::<serde_json::Value>(&stats_json) {
            if let (Some(bytes), Some(total_bytes)) = (
                stats["bytes"].as_f64(),
                stats["totalBytes"].as_f64()
            ) {
                let mb = bytes / 1024.0 / 1024.0;
                let total_mb = total_bytes / 1024.0 / 1024.0;
                let elapsed = start_time.elapsed().as_secs_f64();
                let mb_per_sec = mb / elapsed;
                let percent = (bytes / total_bytes * 100.0).min(100.0);

                info!(
                    "Transferred: {:.1} MB / {:.1} MB, {:.1}%, {:.1} MB/s",
                    mb, total_mb, percent, mb_per_sec
                );
            }

            // Check if transfer is complete
            if stats["finished"].as_bool().unwrap_or(false) {
                break;
            }
        }
    }

    info!("Upload completed");
    librclone::finalize();
    
    Ok(())
}
