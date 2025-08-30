//! System utility Tauri commands
//! 
//! This module contains Tauri commands for system operations like
//! opening URLs, sending notifications, and SSH connections.


use crate::notifications::system::send_notification;

/// Opens a webcam stream in the default browser
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Success or error message
#[tauri::command]
pub fn open_webcam_command(host: String) -> Result<(), String> {
    let webcam_url = format!("http://{}/webcam/?action=stream", host);
    
    // Use system browser to open URL
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open").arg(&webcam_url).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd").args(&["/C", "start", &webcam_url]).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open").arg(&webcam_url).spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Opens the host in the default browser
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * Success or error message
#[tauri::command]
pub fn open_host_in_browser_command(host: String) -> Result<(), String> {
    // Try multiple URL formats
    let urls = vec![
        format!("http://{}", host),
        format!("http://{}:7125", host), // Moonraker default port
        format!("http://{}:8080", host), // Alternative port
    ];
    
    // Use system browser to open URL
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        for url in urls {
            match Command::new("open").arg(&url).spawn() {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
        Err("Failed to open any URL in browser".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        for url in urls {
            match Command::new("cmd").args(&["/C", "start", &url]).spawn() {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
        Err("Failed to open any URL in browser".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        for url in urls {
            match Command::new("xdg-open").arg(&url).spawn() {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
        Err("Failed to open any URL in browser".to_string())
    }
}

/// Opens an SSH connection to the host
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `user` - Username for SSH connection
/// 
/// # Returns
/// * Success or error message
#[tauri::command]
pub fn open_ssh_connection_command(host: String, user: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // Try multiple approaches for macOS
        let ssh_command = format!("ssh {}@{}", user, host);
        
        // First try: AppleScript with Terminal
        let script = format!(
            "tell application \"Terminal\" to do script \"{}\"",
            ssh_command
        );
        
        match Command::new("osascript")
            .args(&["-e", &script])
            .spawn() {
            Ok(_) => return Ok(()),
            Err(_) => {
                // Second try: Direct terminal command
                match Command::new("open")
                    .args(&["-a", "Terminal", "ssh://{}@{}", &user, &host])
                    .spawn() {
                    Ok(_) => return Ok(()),
                    Err(_) => {
                        // Third try: iTerm2 if available
                        let script = format!(
                            "tell application \"iTerm\" to create window with default profile command \"{}\"",
                            ssh_command
                        );
                        Command::new("osascript")
                            .args(&["-e", &script])
                            .spawn()
                            .map_err(|e| format!("Failed to open SSH connection: {}", e))?;
                        return Ok(());
                    }
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "start", "ssh", &format!("{}@{}", user, host)])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("gnome-terminal")
            .args(&["--", "bash", "-c", &format!("ssh {}@{}", user, host)])
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
}

/// Sends a system notification
/// 
/// # Arguments
/// * `title` - Notification title
/// * `body` - Notification body text
/// 
/// # Returns
/// * Success or error message
#[tauri::command]
pub fn send_system_notification_command(title: String, body: String) -> Result<(), String> {
    send_notification(&title, &body);
    Ok(())
}
