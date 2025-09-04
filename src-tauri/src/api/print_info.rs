//! Print information API functions
//! 
//! This module provides functions to query printer objects and extract
//! print job information and progress data.

use crate::error::{MoonrakerResult, MoonrakerError};
use crate::models::print_info::{PrinterObjectsQuery, PrintJobInfo, PrintProgress};
use crate::api::client::create_client;

/// Gets comprehensive print information from printer objects
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `port` - Moonraker port (default: 7125)
/// 
/// # Returns
/// * PrintJobInfo with current print status and progress
pub async fn get_print_info(host: &str, port: Option<u16>) -> MoonrakerResult<Option<PrintJobInfo>> {
    let port = port.unwrap_or(7125);
    let client = create_client().await?;
    
    let url = format!("http://{}:{}/printer/objects/query?print_stats&virtual_sdcard&toolhead&extruder", host, port);
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to query printer objects: {}", e))?;
    
    if !response.status().is_success() {
        return Ok(None);
    }
    
    let response_text = response.text().await
        .map_err(|e| format!("Failed to get response text: {}", e))?;
    
    let data: PrinterObjectsQuery = serde_json::from_str(&response_text)
        .map_err(|e| MoonrakerError::Api(format!("Failed to parse printer objects: {}", e)))?;
    
    // Extract print information
    let print_stats = &data.result.status.print_stats;
    let virtual_sdcard = &data.result.status.virtual_sdcard;
    
    if let (Some(stats), Some(sdcard)) = (print_stats, virtual_sdcard) {
        // Calculate progress percentage
        let progress = sdcard.progress * 100.0;
        
        // Calculate durations with fallbacks
        let print_duration = stats.print_duration.unwrap_or(0.0);
        let total_duration = stats.total_duration.unwrap_or(0.0);
        
        // Get filename from print_stats with fallback
        let filename = stats.filename.clone().unwrap_or_else(|| "Unknown".to_string());
        
        // Get layer info
        let current_layer = stats.info.as_ref().and_then(|info| info.current_layer);
        let total_layers = stats.info.as_ref().and_then(|info| info.total_layer);
        
        // Create print progress info
        let progress_info = PrintProgress {
            progress,
            print_duration,
            total_duration,
            current_layer,
            total_layers,
            height: None, // Not available in basic API
            total_height: None, // Not available in basic API
        };
        
        // Create print job info
        let print_job = PrintJobInfo {
            filename,
            total_size: sdcard.file_size, // Available in virtual_sdcard
            progress: progress_info,
            start_time: 0.0, // Not available in this API
            estimated_completion: None,
            status: stats.state.clone().unwrap_or_else(|| "printing".to_string()),
        };
        
        Ok(Some(print_job))
    } else {
        Ok(None)
    }
}

/// Gets print progress percentage for display in status
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `port` - Moonraker port (default: 7125)
/// 
/// # Returns
/// * Progress percentage (0.0 - 100.0) or None if not printing
pub async fn get_print_progress(host: &str, port: Option<u16>) -> MoonrakerResult<Option<f64>> {
    let print_info = get_print_info(host, port).await?;
    
    if let Some(info) = print_info {
        Ok(Some(info.progress.progress))
    } else {
        Ok(None)
    }
}

/// Formats print duration in human readable format
/// 
/// # Arguments
/// * `seconds` - Duration in seconds
/// 
/// # Returns
/// * Formatted duration string (e.g., "2h 15m 30s")
pub fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
