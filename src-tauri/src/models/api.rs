//! Moonraker API data structures
//! 
//! This module contains all data structures used for communication with
//! the Moonraker API, including server info, printer info, and status flags.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Server information response from Moonraker API
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerServerInfo {
    pub result: ServerInfoResult,
}

/// Detailed server information including Klippy state and components
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfoResult {
    pub klippy_connected: bool,
    pub klippy_state: String,
    pub components: Vec<String>,
    pub failed_components: Vec<String>,
    pub registered_directories: Vec<String>,
    pub warnings: Vec<String>,
    pub websocket_count: i32,
    pub moonraker_version: String,
    pub api_version: Vec<i32>,
    #[serde(rename = "api_version_string")]
    pub api_version_string: Option<String>,
    #[serde(rename = "missing_klippy_requirements")]
    pub missing_klippy_requirements: Option<Vec<String>>,
}

/// Printer information response from Moonraker API
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerPrinterInfo {
    pub result: PrinterInfoResult,
}

/// Detailed printer information
#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterInfoResult {
    pub state: String,
    pub state_message: String,
    pub hostname: Option<String>,
    pub software_version: Option<String>,
    pub cpu_info: Option<String>,
    pub klipper_path: Option<String>,
    pub python_path: Option<String>,
    pub log_file: Option<String>,
    pub config_file: Option<String>,
}

/// Printer objects response from Moonraker API
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerPrinterObjects {
    pub result: PrinterObjectsResult,
}

/// Printer objects result containing various printer state objects
#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterObjectsResult {
    pub objects: HashMap<String, PrinterObject>,
}

/// Individual printer object with dynamic value
#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterObject {
    pub value: serde_json::Value,
}

/// Printer status flags from Moonraker API state.flags
/// 
/// These flags indicate the current state of the 3D printer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterFlags {
    /// Printer is operational and ready
    pub operational: bool,
    /// Print is currently paused
    pub paused: bool,
    /// Print is currently running
    pub printing: bool,
    /// Print is being cancelled
    pub cancelling: bool,
    /// Print is being paused
    pub pausing: bool,
    /// Print is being resumed
    #[serde(default)]
    pub resuming: bool,
    /// SD card is ready
    #[serde(rename = "sdReady")]
    #[serde(default)]
    pub sd_ready: bool,
    /// Printer is in error state
    pub error: bool,
    /// Printer is ready for printing
    pub ready: bool,
    /// Printer is closed or in error state
    #[serde(rename = "closedOrError")]
    pub closed_or_error: bool,
}

impl PrinterFlags {
    /// Determines the printer status based on flags priority
    /// 
    /// Priority order: cancelling > error > paused > printing > ready > standby
    pub fn get_status(&self) -> &'static str {
        if self.cancelling {
            "cancelling"
        } else if self.error {
            "error"
        } else if self.paused {
            "paused"
        } else if self.printing {
            "printing"
        } else if self.ready {
            "standby"
        } else {
            "standby"
        }
    }
}
