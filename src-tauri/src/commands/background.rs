//! Background monitoring Tauri commands
//! 
//! This module contains Tauri commands for managing background monitoring

use tauri::{AppHandle, State};
use crate::background_monitor::BackgroundMonitorState;

/// Starts the background monitoring process
#[tauri::command]
pub async fn start_background_monitoring_command(
    app_handle: AppHandle,
    state: State<'_, BackgroundMonitorState>,
    interval_seconds: u64,
) -> Result<(), String> {
    state.start(app_handle, interval_seconds).await
}

/// Stops the background monitoring process
#[tauri::command]
pub fn stop_background_monitoring_command(
    state: State<'_, BackgroundMonitorState>,
) -> Result<(), String> {
    state.stop();
    Ok(())
}

/// Gets the status of the background monitoring process
#[tauri::command]
pub fn get_background_monitoring_status_command(
    state: State<'_, BackgroundMonitorState>,
) -> Result<bool, String> {
    Ok(state.is_running())
}
