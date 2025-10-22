//! HTTP client utilities for Moonraker API communication
//! 
//! This module provides a configured HTTP client and utility functions
//! for making requests to Moonraker printers.

use std::time::Duration;
use reqwest::Client;
use crate::error::{MoonrakerError, MoonrakerResult};
use crate::models::config::{DEFAULT_TIMEOUT_SECONDS, MOONRAKER_PORT};

/// Creates a configured HTTP client for Moonraker API requests
/// 
/// The client is configured with:
/// - 5 second timeout for all requests
/// - Proper headers for JSON communication
/// - Connection pooling for efficiency
pub async fn create_client() -> MoonrakerResult<Client> {
    Client::builder()
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
        .build()
        .map_err(MoonrakerError::Network)
}

/// Builds a Moonraker API URL for a given host and endpoint
/// 
/// # Arguments
/// * `host` - Host IP address or hostname
/// * `endpoint` - API endpoint (e.g., "server/info", "printer/info")
/// 
/// # Returns
/// * Full URL for the API request
pub fn build_moonraker_url(host: &str, endpoint: &str) -> String {
    format!("http://{}:{}/{}", host, MOONRAKER_PORT, endpoint)
}

/// Makes a GET request to a Moonraker API endpoint
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `endpoint` - API endpoint
/// 
/// # Returns
/// * JSON response as serde_json::Value
pub async fn get_moonraker_endpoint(host: &str, endpoint: &str) -> MoonrakerResult<serde_json::Value> {
    let client = create_client().await?;
    let url = build_moonraker_url(host, endpoint);
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(MoonrakerError::Network)?;

    if response.status().is_success() {
        let data: serde_json::Value = response
            .json()
            .await
            .map_err(MoonrakerError::Network)?;
        Ok(data)
    } else {
        Err(MoonrakerError::Api(format!(
            "HTTP {}: {}",
            response.status(),
            response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
        )))
    }
}

/// Makes a POST request to a Moonraker API endpoint
/// 
/// # Arguments
/// * `host` - Host IP address
/// * `endpoint` - API endpoint
/// * `body` - Optional JSON body for the request
/// 
/// # Returns
/// * JSON response as serde_json::Value
pub async fn post_moonraker_endpoint(
    host: &str,
    endpoint: &str,
    body: Option<serde_json::Value>,
) -> MoonrakerResult<serde_json::Value> {
    let client = create_client().await?;
    let url = build_moonraker_url(host, endpoint);
    
    let mut request = client.post(&url);
    
    if let Some(body_data) = body {
        request = request.json(&body_data);
    }
    
    let response = request
        .send()
        .await
        .map_err(MoonrakerError::Network)?;

    let status = response.status();
    if status.is_success() {
        let data: serde_json::Value = response
            .json()
            .await
            .map_err(MoonrakerError::Network)?;
        Ok(data)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(MoonrakerError::Api(format!(
            "HTTP {}: {}",
            status,
            error_text
        )))
    }
}
