//! Print information Tauri commands
//! 
//! This module contains Tauri commands for retrieving print job information
//! and progress data from Moonraker printers.

use crate::error::error_to_string;
use crate::api::print_info::{get_print_info, get_print_progress, format_duration};
use crate::models::print_info::PrintJobInfo;

/// Gets comprehensive print information for a host
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `port` - Moonraker port (optional, default: 7125)
/// 
/// # Returns
/// * PrintJobInfo with current print status and progress, or None if not printing
#[tauri::command]
pub async fn get_print_info_command(host: String, port: Option<u16>) -> Result<Option<PrintJobInfo>, String> {
    get_print_info(&host, port).await.map_err(error_to_string)
}

/// Gets print progress percentage for a host
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `port` - Moonraker port (optional, default: 7125)
/// 
/// # Returns
/// * Progress percentage (0.0 - 100.0) or None if not printing
#[tauri::command]
pub async fn get_print_progress_command(host: String, port: Option<u16>) -> Result<Option<f64>, String> {
    get_print_progress(&host, port).await.map_err(error_to_string)
}

/// Formats duration in human readable format
/// 
/// # Arguments
/// * `seconds` - Duration in seconds
/// 
/// # Returns
/// * Formatted duration string (e.g., "2h 15m 30s")
#[tauri::command]
pub fn format_duration_command(seconds: f64) -> Result<String, String> {
    Ok(format_duration(seconds))
}
