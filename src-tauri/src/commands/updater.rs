//! Update checking Tauri commands
//! 
//! This module contains Tauri commands for checking application updates
//! and managing the update process.

use crate::error::error_to_string;
use crate::updater::{GitHubUpdater, UpdateCheckResult};

/// Checks for available updates
/// 
/// # Returns
/// * UpdateCheckResult with update information
#[tauri::command]
pub async fn check_for_updates_command() -> Result<UpdateCheckResult, String> {
    let updater = GitHubUpdater::new();
    updater.check_for_updates().await.map_err(error_to_string)
}

/// Gets the repository URL
/// 
/// # Returns
/// * Repository URL as string
#[tauri::command]
pub fn get_repository_url_command() -> Result<String, String> {
    let updater = GitHubUpdater::new();
    Ok(updater.get_repository_url())
}

/// Gets the releases URL
/// 
/// # Returns
/// * Releases URL as string
#[tauri::command]
pub fn get_releases_url_command() -> Result<String, String> {
    let updater = GitHubUpdater::new();
    Ok(updater.get_releases_url())
}
