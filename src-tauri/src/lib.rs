//! Moonraker Host Scanner - Rust Backend Library
//! 
//! This module provides the backend functionality for the Moonraker Host Scanner
//! desktop application, including network scanning, Moonraker API integration,
//! and system notifications.
//! 
//! # Architecture
//! 
//! The library is organized into the following modules:
//! 
//! - `models/` - Data structures and types
//! - `api/` - API client and communication functions
//! - `network/` - Network scanning and utilities
//! - `commands/` - Tauri command handlers
//! - `notifications/` - System notification functions
//! - `error.rs` - Error handling and types
//! 
//! # Features
//! 
//! - Network discovery and host scanning
//! - Moonraker API communication
//! - Printer status monitoring
//! - System notifications
//! - Cross-platform compatibility

// Module declarations
pub mod error;
pub mod models;
pub mod api;
pub mod network;
pub mod commands;
pub mod notifications;
pub mod updater;

// Re-export commonly used types
pub use error::{MoonrakerError, MoonrakerResult};
pub use models::*;

// Tauri application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Scan commands
            commands::scan::scan_network_command,
            commands::scan::get_host_info_command,
            commands::scan::check_host_status_command,
            
            // Printer commands
            commands::printer::control_printer_command,
            commands::printer::get_printer_status_command,
            
            // Print info commands
            commands::print_info::get_print_info_command,
            commands::print_info::get_print_progress_command,
            commands::print_info::format_duration_command,
            
            // System commands
            commands::system::open_webcam_command,
            commands::system::open_host_in_browser_command,
            commands::system::open_ssh_connection_command,
            commands::system::send_system_notification_command,
            commands::system::open_url_in_browser_command,
            commands::system::check_notification_status_command,
            
            // Updater commands
            commands::updater::check_for_updates_command,
            commands::updater::get_repository_url_command,
            commands::updater::get_releases_url_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
