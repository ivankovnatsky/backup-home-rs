//! A tool for creating and uploading backups of user directories.
//!
//! This crate provides functionality to:
//! - Create platform-specific backups (ZIP for Windows, tar.gz for macOS/Linux)
//! - Upload backups to cloud storage using rclone
//! - Handle platform-specific exclude patterns

pub mod backup;
pub mod platform;
pub mod upload;
