//! Print job information and progress models
//! 
//! This module contains data structures for tracking print job progress
//! and detailed information about current printing tasks.

use serde::{Deserialize, Serialize};

/// Print job progress information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintProgress {
    /// Current print progress percentage (0.0 - 100.0)
    pub progress: f64,
    /// Time elapsed since print start
    pub print_duration: f64,
    /// Total estimated time for the print
    pub total_duration: f64,
    /// Current layer being printed
    pub current_layer: Option<u32>,
    /// Total layers in the print
    pub total_layers: Option<u32>,
    /// Current height in mm
    pub height: Option<f64>,
    /// Total height in mm
    pub total_height: Option<f64>,
}

/// Print job information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintJobInfo {
    /// Name of the file being printed
    pub filename: String,
    /// Total size of the file in bytes
    pub total_size: u64,
    /// Print progress information
    pub progress: PrintProgress,
    /// Print start time (Unix timestamp)
    pub start_time: f64,
    /// Estimated completion time (Unix timestamp)
    pub estimated_completion: Option<f64>,
    /// Print status (printing, paused, completed, etc.)
    pub status: String,
}

/// Moonraker printer objects query response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterObjectsQuery {
    pub result: PrinterObjectsData,
}

/// Printer objects data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterObjectsData {
    /// Event time
    pub eventtime: f64,
    /// Status information
    pub status: PrinterStatus,
}

/// Printer status information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterStatus {
    /// Print job information
    #[serde(rename = "print_stats")]
    pub print_stats: Option<PrintStats>,
    /// Virtual SD card information
    #[serde(rename = "virtual_sdcard")]
    pub virtual_sdcard: Option<VirtualSDCard>,
    /// Toolhead information
    pub toolhead: Option<Toolhead>,
    /// Extruder information
    pub extruder: Option<Extruder>,
}

/// Print statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintStats {
    /// Current print filename
    pub filename: Option<String>,
    /// Total print time in seconds
    pub total_duration: Option<f64>,
    /// Print time in seconds
    pub print_duration: Option<f64>,
    /// Filament used in mm
    pub filament_used: Option<f64>,
    /// Print state
    pub state: Option<String>,
    /// Print message
    pub message: Option<String>,
    /// Print info with layer information
    pub info: Option<PrintInfo>,
}

/// Print info with layer details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintInfo {
    /// Total layers in the print
    pub total_layer: Option<u32>,
    /// Current layer being printed
    pub current_layer: Option<u32>,
}

/// Virtual SD card information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualSDCard {
    /// File path
    pub file_path: String,
    /// Progress percentage (0.0 - 1.0)
    pub progress: f64,
    /// Is SD card ready
    pub is_active: bool,
    /// Current file position
    pub file_position: u64,
    /// Total file size
    pub file_size: u64,
}

/// Toolhead information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Toolhead {
    /// Current position [x, y, z, e]
    pub position: [f64; 4],
    /// Current speed
    pub speed: Option<f64>,
    /// Current acceleration
    pub acceleration: Option<f64>,
    /// Current jerk
    pub jerk: Option<f64>,
}

/// Extruder information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extruder {
    /// Current temperature
    pub temperature: f64,
    /// Target temperature
    pub target: f64,
    /// Power (0.0 - 1.0)
    pub power: f64,
    /// Can extrude
    pub can_extrude: bool,
}
