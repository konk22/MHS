//! System notification functionality
//! 
//! This module provides functions for sending native system notifications
//! across different platforms.

use notify_rust::Notification;

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
            // Try to set the application name, but don't fail if it doesn't work
            let _ = notify_rust::set_application("com.apple.Safari");
        });
    }
    
    match Notification::new()
        .summary(title)
        .body(body)
        .icon("printer") // Printer icon
        .show() {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to send notification: {}", e),
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
