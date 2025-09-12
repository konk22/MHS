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
pub mod background_monitor;
pub mod telegram;

// Re-export commonly used types
pub use error::{MoonrakerError, MoonrakerResult};
pub use models::*;

// Tauri application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::Manager;
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Hide window instead of closing when user clicks X
                    println!("Window close requested - hiding to tray");
                    window.hide().unwrap();
                    // Ensure it stays hidden from taskbar
                    window.set_skip_taskbar(true).unwrap();
                    // Set accessory activation policy on macOS to hide from Dock
                    #[cfg(target_os = "macos")]
                    {
                        let _ = window.app_handle().set_activation_policy(tauri::ActivationPolicy::Accessory);
                    }
                    api.prevent_close();
                }
                tauri::WindowEvent::Focused(focused) => {
                    if *focused {
                        // Show in taskbar when window is focused
                        window.set_skip_taskbar(false).unwrap();
                    }
                }
                _ => {}
            }
        })
        .manage(background_monitor::BackgroundMonitorState::new())
        .manage(commands::telegram::TelegramBotState::new())
        .setup(|app| {
            // Create system tray with menu
            use tauri::{
                menu::{Menu, MenuItem},
                tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
                Manager,
            };

            // Create menu items
            let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            // Create menu
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .tooltip("Moonraker Host Scanner")
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            println!("Show window menu item clicked");
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.unminimize();
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Show in taskbar when window is shown
                                let _ = window.set_skip_taskbar(false);
                                // Restore normal activation policy on macOS
                                #[cfg(target_os = "macos")]
                                {
                                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                                }
                            }
                        }
                        "hide" => {
                            println!("Hide window menu item clicked");
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                                // Keep hidden from taskbar when window is hidden
                                let _ = window.set_skip_taskbar(true);
                                // Set accessory activation policy on macOS to hide from Dock
                                #[cfg(target_os = "macos")]
                                {
                                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                                }
                            }
                        }
                        "quit" => {
                            println!("Quit menu item clicked");
                            app.exit(0);
                        }
                        _ => {
                            println!("Unknown menu item: {:?}", event.id);
                        }
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            println!("Tray icon left clicked");
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.unminimize();
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Show in taskbar when window is shown
                                let _ = window.set_skip_taskbar(false);
                                // Restore normal activation policy on macOS
                                #[cfg(target_os = "macos")]
                                {
                                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                                }
                            }
                        }
                        _ => {
                            // Ignore other events to reduce console spam
                        }
                    }
                })
                .build(app)?;

            println!("Application initialized successfully with system tray");
            Ok(())
        })
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
            
            // Background monitoring commands
            commands::background::start_background_monitoring_command,
            commands::background::stop_background_monitoring_command,
            commands::background::get_background_monitoring_status_command,
            
            // Telegram bot commands
            commands::telegram::start_telegram_bot,
            commands::telegram::stop_telegram_bot,
            commands::telegram::get_telegram_bot_status,
            commands::telegram::start_telegram_registration,
            commands::telegram::stop_telegram_registration,
            commands::telegram::is_telegram_registration_active,
            commands::telegram::get_telegram_users,
            commands::telegram::remove_telegram_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
