//! Application updater module
//! 
//! This module provides functionality to check for application updates
//! by querying the GitHub repository for new releases.

pub mod github;
pub mod models;

pub use github::*;
pub use models::*;
