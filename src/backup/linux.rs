use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::time::{Duration, Instant};
use flate2::Compression;
use gzp::{
    deflate::Gzip,
    ZBuilder,
};
use tar::Builder;
use tracing::{info, debug, warn};

pub fn create_archive(source: &str, backup_path: &PathBuf, compression_level: u32) -> Result<()> {
    let file = File::create(backup_path)?;
    let compression = Compression::new(compression_level.clamp(1, 9));
    
    let gzip = ZBuilder::<Gzip, _>::new()
        .num_threads(num_cpus::get())
        .compression_level(compression)
        .from_writer(file);
    
    let mut archive = Builder::new(gzip);

    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(5);
    
    let source_path = std::path::Path::new(source);
    let exclude_patterns: Vec<glob::Pattern> = crate::platform::get_exclude_patterns()
        .iter()
        .filter_map(|pattern| {
            match glob::Pattern::new(pattern) {
                Ok(p) => Some(p),
                Err(e) => {
                    warn!("Invalid exclude pattern '{}': {}", pattern, e);
                    None
                }
            }
        })
        .collect();
    
    info!("Using exclude patterns: [{}]", crate::platform::get_exclude_patterns().join(", "));
    
    let mut total_files = 0;
    let mut excluded_files = 0;
    
    for entry in walkdir::WalkDir::new(source_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            total_files += 1;
            let path = e.path();
            
            // Get path relative to source directory for pattern matching
            let relative_path = match path.strip_prefix(source_path) {
                Ok(p) => {
                    // Ensure the path starts with "./" for pattern matching
                    format!("./{}", p.display())
                },
                Err(_) => return true, // If strip_prefix fails, include the file
            };
            
            let should_exclude = exclude_patterns.iter().any(|pattern| {
                let matches = pattern.matches(&relative_path);
                if matches {
                    excluded_files += 1;
                    debug!("Excluding: {} (matched pattern {})", relative_path, pattern);
                }
                matches
            });
            
            if !should_exclude {
                debug!("Including: {}", relative_path);
            }
            
            !should_exclude
        })
    {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(source_path)?;
        
        debug!("Processing: {}", relative_path.display());
        
        if path.is_file() {
            archive.append_path_with_name(path, relative_path)?;
        } else if path.is_dir() {
            archive.append_dir(relative_path, path)?;
        }

        if last_update.elapsed() >= update_interval {
            if let Ok(metadata) = backup_path.metadata() {
                let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
                let elapsed = start_time.elapsed().as_secs_f64();
                let mb_per_sec = size_mb / elapsed;
                
                info!(
                    "Archive size: {:.2} MB ({:.2} MB/s)",
                    size_mb,
                    mb_per_sec
                );
            }
            last_update = Instant::now();
        }
    }

    if let Ok(metadata) = backup_path.metadata() {
        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
        let elapsed = start_time.elapsed().as_secs_f64();
        let mb_per_sec = size_mb / elapsed;
        
        info!(
            "Final archive size: {:.2} MB (average speed: {:.2} MB/s)",
            size_mb,
            mb_per_sec
        );
    }

    archive.finish()?;
    Ok(())
} 
