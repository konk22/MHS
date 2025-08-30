//! Moonraker API communication functions
//! 
//! This module contains functions for communicating with Moonraker API endpoints,
//! including server info, printer info, and status queries.

use crate::error::MoonrakerResult;
use crate::models::api::{
    MoonrakerServerInfo,
    MoonrakerPrinterInfo,
    MoonrakerPrinterObjects,
    PrinterFlags,
};
use crate::api::client::get_moonraker_endpoint;

/// Checks if Moonraker API is available on the specified host
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Server information if API is available
pub async fn check_moonraker_api(host: &str) -> MoonrakerResult<MoonrakerServerInfo> {
    let data = get_moonraker_endpoint(host, "server/info").await?;
    
    match serde_json::from_value(data) {
        Ok(server_info) => Ok(server_info),
        Err(_) => {
            // If parsing fails, return a default server info
            Ok(MoonrakerServerInfo {
                result: crate::models::api::ServerInfoResult {
                    klippy_connected: false,
                    klippy_state: "disconnected".to_string(),
                    components: vec![],
                    failed_components: vec![],
                    registered_directories: vec![],
                    warnings: vec![],
                    websocket_count: 0,
                    moonraker_version: "unknown".to_string(),
                    api_version: vec![1, 0, 0],
                    api_version_string: None,
                    missing_klippy_requirements: None,
                }
            })
        }
    }
}

/// Gets printer information from Moonraker API
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Printer information
pub async fn get_printer_info(host: &str) -> MoonrakerResult<MoonrakerPrinterInfo> {
    let data = get_moonraker_endpoint(host, "printer/info").await?;
    
    match serde_json::from_value(data) {
        Ok(printer_info) => Ok(printer_info),
        Err(_) => {
            // If parsing fails, return a default printer info
            Ok(MoonrakerPrinterInfo {
                result: crate::models::api::PrinterInfoResult {
                    state: "standby".to_string(),
                    state_message: "Printer info unavailable".to_string(),
                    hostname: Some(host.to_string()),
                    software_version: None,
                    cpu_info: None,
                    klipper_path: None,
                    python_path: None,
                    log_file: None,
                    config_file: None,
                }
            })
        }
    }
}

/// Gets printer objects from Moonraker API
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Printer objects information
pub async fn get_printer_objects(host: &str) -> MoonrakerResult<MoonrakerPrinterObjects> {
    let data = get_moonraker_endpoint(host, "printer/objects/query?print_stats").await?;
    
    match serde_json::from_value(data) {
        Ok(printer_objects) => Ok(printer_objects),
        Err(_) => {
            // If parsing fails, return a default printer objects
            Ok(MoonrakerPrinterObjects {
                result: crate::models::api::PrinterObjectsResult {
                    objects: std::collections::HashMap::new(),
                }
            })
        }
    }
}

/// Gets printer status flags from Moonraker API
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Printer status flags
pub async fn get_printer_flags(host: &str) -> MoonrakerResult<PrinterFlags> {
    let data = get_moonraker_endpoint(host, "api/printer").await?;
    
    // Extract flags from the state object
    if let Some(state) = data.get("state") {
        if let Some(flags) = state.get("flags") {
            match serde_json::from_value(flags.clone()) {
                Ok(printer_flags) => Ok(printer_flags),
                Err(_) => {
                    // If parsing fails, try to create default flags
                    // Return default flags instead of error
                    Ok(PrinterFlags {
                        operational: false,
                        paused: false,
                        printing: false,
                        cancelling: false,
                        pausing: false,
                        resuming: false,
                        sd_ready: false,
                        error: false,
                        ready: true,
                        closed_or_error: false,
                    })
                }
            }
        } else {
            // No flags found, return default flags
            Ok(PrinterFlags {
                operational: false,
                paused: false,
                printing: false,
                cancelling: false,
                pausing: false,
                resuming: false,
                sd_ready: false,
                error: false,
                ready: true,
                closed_or_error: false,
            })
        }
    } else {
        // No state found, return default flags
        Ok(PrinterFlags {
            operational: false,
            paused: false,
            printing: false,
            cancelling: false,
            pausing: false,
            resuming: false,
            sd_ready: false,
            error: false,
            ready: true,
            closed_or_error: false,
        })
    }
}

/// Gets comprehensive printer status information
/// 
/// This function combines multiple API calls to get complete printer status
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Combined printer status information
pub async fn get_comprehensive_printer_status(host: &str) -> MoonrakerResult<serde_json::Value> {
    let printer_info = get_printer_info(host).await?;
    let printer_objects = get_printer_objects(host).await?;
    
    let status = serde_json::json!({
        "printer_info": printer_info.result,
        "printer_objects": printer_objects.result,
    });
    
    Ok(status)
}
