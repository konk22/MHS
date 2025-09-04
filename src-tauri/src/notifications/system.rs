//! System notification functionality
//! 
//! This module provides functions for sending native system notifications
//! across different platforms.

use notify_rust::Notification;

/// Checks notification permissions on macOS
#[cfg(target_os = "macos")]
pub fn check_notification_permissions() -> Result<(), String> {
    use std::process::Command;
    
    // Check if notifications are enabled in System Preferences
    let output = Command::new("osascript")
        .args(&["-e", "tell application \"System Events\" to get properties of process \"SystemUIServer\""])
        .output()
        .map_err(|e| format!("Failed to check system permissions: {}", e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err("System permissions check failed".to_string())
    }
}

/// Sends a system notification using the platform's native notification system
/// 
/// # Arguments
/// * `title` - Notification title
/// * `body` - Notification body text
pub fn send_notification(title: &str, body: &str) {
    // On macOS, we need to set the app name only once
    #[cfg(target_os = "macos")]
    {
        use std::sync::Once;
        static INIT: Once = Once::new();
        
        INIT.call_once(|| {
            // Check permissions first
            if let Err(_e) = check_notification_permissions() {
            }
            
            // Use the correct bundle identifier from tauri.conf.json
            match notify_rust::set_application("com.tormyhseviv.moonrakerhostscanner") {
                Ok(_) => {},
                Err(_) => {},
            }
        });
    }
    
    match Notification::new()
        .summary(title)
        .body(body)
        .icon("printer") // Printer icon
        .show() {
        Ok(_) => {},
        Err(_) => {},
    }
}

/// Sends a notification about printer status change
/// 
/// # Arguments
/// * `hostname` - Printer hostname
/// * `old_status` - Previous status
/// * `new_status` - New status
pub fn send_status_change_notification(hostname: &str, old_status: &str, new_status: &str) {
    let title = "Printer Status Changed";
    let body = format!("{}: {} → {}", hostname, old_status, new_status);
    send_notification(title, &body);
}

/// Sends a notification about printer discovery
/// 
/// # Arguments
/// * `hostname` - Printer hostname
/// * `ip_address` - Printer IP address
pub fn send_printer_discovered_notification(hostname: &str, ip_address: &str) {
    let title = "New Printer Discovered";
    let body = format!("{} ({})", hostname, ip_address);
    send_notification(title, &body);
}

/// Sends a notification about printer going offline
/// 
/// # Arguments
/// * `hostname` - Printer hostname
/// * `ip_address` - Printer IP address
pub fn send_printer_offline_notification(hostname: &str, ip_address: &str) {
    let title = "Printer Offline";
    let body = format!("{} ({}) is no longer responding", hostname, ip_address);
    send_notification(title, &body);
}
