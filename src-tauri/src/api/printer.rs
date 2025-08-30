//! Printer control functions
//! 
//! This module contains functions for controlling 3D printers through
//! the Moonraker API, including print operations and emergency controls.

use crate::error::{MoonrakerError, MoonrakerResult};
use crate::api::client::post_moonraker_endpoint;

/// Available printer control actions
#[derive(Debug, Clone, Copy)]
pub enum PrinterAction {
    /// Start a print job
    Start,
    /// Pause the current print
    Pause,
    /// Cancel the current print
    Cancel,
    /// Emergency stop the printer
    EmergencyStop,
}

impl PrinterAction {
    /// Converts the action to its API endpoint
    pub fn to_endpoint(&self) -> &'static str {
        match self {
            PrinterAction::Start => "printer/print/start",
            PrinterAction::Pause => "printer/print/pause",
            PrinterAction::Cancel => "printer/print/cancel",
            PrinterAction::EmergencyStop => "printer/emergency_stop",
        }
    }

    /// Converts string action to PrinterAction
    pub fn from_string(action: &str) -> MoonrakerResult<Self> {
        match action {
            "start" => Ok(PrinterAction::Start),
            "pause" => Ok(PrinterAction::Pause),
            "cancel" => Ok(PrinterAction::Cancel),
            "emergency_stop" => Ok(PrinterAction::EmergencyStop),
            _ => Err(MoonrakerError::Api(format!("Unknown printer action: {}", action))),
        }
    }
}

/// Controls the printer with the specified action
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `action` - Printer action to perform
/// 
/// # Returns
/// * API response as JSON
pub async fn control_printer(host: &str, action: PrinterAction) -> MoonrakerResult<serde_json::Value> {
    let endpoint = action.to_endpoint();
    post_moonraker_endpoint(host, endpoint, None).await
}

/// Controls the printer using a string action
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `action` - Action string (start, pause, cancel, emergency_stop)
/// 
/// # Returns
/// * API response as JSON
pub async fn control_printer_with_string(host: &str, action: &str) -> MoonrakerResult<serde_json::Value> {
    let printer_action = PrinterAction::from_string(action)?;
    control_printer(host, printer_action).await
}
