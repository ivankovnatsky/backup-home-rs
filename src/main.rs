use backup_home::backup;
use backup_home::upload;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Source directory to backup (defaults to home directory)
    #[arg(short, long)]
    source: Option<String>,

    /// Destination path for rclone (e.g., "drive:", "gdrive:backup/home")
    #[arg(short, long)]
    destination: String,

    /// Compression level (0-9, default: 6)
    #[arg(short, long)]
    compression: Option<u32>,

    /// Preview what would be done without actually doing it
    #[arg(long)]
    preview: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Get source directory or default to home
    let source = cli.source.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .to_string_lossy()
            .to_string()
    });

    if cli.preview {
        println!("\nPreview summary:");
        println!("---------------");
        println!("Source: {}", source);
        println!("Destination: {}", cli.destination);
        println!("Compression level: {}", cli.compression.unwrap_or(6));
        println!("\nThis would:");
        println!("1. Create backup archive of: {}", source);
        println!("2. Upload to: {}", cli.destination);
        println!("3. Clean up temporary files");
        return Ok(());
    }

    // Create backup
    let backup_path = backup::create_backup(&source, cli.compression)?;

    // Upload backup
    upload::upload_to_rclone(&backup_path, &cli.destination).await?;

    // Cleanup
    std::fs::remove_file(backup_path)?;

    Ok(())
}
