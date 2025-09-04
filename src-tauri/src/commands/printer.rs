//! Printer control Tauri commands
//! 
//! This module contains Tauri commands for controlling 3D printers.

use crate::error::error_to_string;
use crate::api::printer::control_printer_with_string;
use crate::api::moonraker::get_comprehensive_printer_status;

/// Controls the printer with the specified action
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `action` - Action to perform (start, pause, resume, cancel, emergency_stop)
/// 
/// # Returns
/// * API response as JSON
#[tauri::command]
pub async fn control_printer_command(host: String, action: String) -> Result<serde_json::Value, String> {
    control_printer_with_string(&host, &action)
        .await
        .map_err(error_to_string)
}

/// Gets comprehensive printer status information
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Combined printer status information
#[tauri::command]
pub async fn get_printer_status_command(host: String) -> Result<serde_json::Value, String> {
    get_comprehensive_printer_status(&host)
        .await
        .map_err(error_to_string)
}
