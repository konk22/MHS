use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::telegram::TelegramBot;
use crate::models::TelegramUser;
use crate::models::config::AppSettings;

pub struct TelegramBotState {
    pub bot: Arc<Mutex<Option<TelegramBot>>>,
    pub hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    pub bot_token: Arc<Mutex<Option<String>>>,
}

impl TelegramBotState {
    pub fn new() -> Self {
        Self {
            bot: Arc::new(Mutex::new(None)),
            hosts: Arc::new(Mutex::new(Vec::new())),
            bot_token: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn start_telegram_bot(
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let mut bot_guard = state.bot.lock().await;
    
    // Stop existing bot if running
    if let Some(ref bot) = *bot_guard {
        if bot.is_running() {
            let _ = bot.stop().await;
        }
    }
    
    let token_guard = state.bot_token.lock().await;
    let bot_token = token_guard.as_ref()
        .ok_or("Bot token not set. Please set the token first.")?;
    
    // Create and start new bot
    let bot = TelegramBot::new(bot_token.clone(), state.hosts.clone()).await?;
    bot.start().await?;
    
    *bot_guard = Some(bot);
    
    Ok("Telegram bot started successfully".to_string())
}

#[tauri::command]
pub async fn stop_telegram_bot(
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let mut bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        bot.stop().await?;
        *bot_guard = None;
        Ok("Telegram bot stopped successfully".to_string())
    } else {
        Err("No bot is currently running".to_string())
    }
}

#[tauri::command]
pub async fn get_telegram_bot_status(
    state: State<'_, TelegramBotState>,
) -> Result<bool, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        Ok(bot.is_running())
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn start_telegram_registration(
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        bot.start_registration().await
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn stop_telegram_registration(
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        bot.stop_registration().await?;
        Ok("Registration stopped successfully".to_string())
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn get_telegram_users(
    _state: State<'_, TelegramBotState>,
) -> Result<Vec<TelegramUser>, String> {
    // Load users from config file
    let settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    Ok(settings.telegram.registered_users)
}

#[tauri::command]
pub async fn remove_telegram_user(
    user_id: i64,
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    // Load current users from config
    let mut settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    
    // Remove user from the list
    settings.telegram.registered_users.retain(|user| user.user_id != user_id);
    
    // Save updated users to config
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    
    // Also remove from bot if it's running
    let bot_guard = state.bot.lock().await;
    if let Some(ref bot) = *bot_guard {
        let _ = bot.remove_user(user_id).await; // Ignore errors from bot
    }
    
    Ok("User removed successfully".to_string())
}

#[tauri::command]
pub async fn is_telegram_registration_active(
    state: State<'_, TelegramBotState>,
) -> Result<bool, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        Ok(bot.is_registration_active().await)
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn get_telegram_hosts(
    state: State<'_, TelegramBotState>,
) -> Result<Vec<crate::models::HostInfo>, String> {
    let hosts = state.hosts.lock().await;
    Ok(hosts.clone())
}

#[tauri::command]
pub async fn update_telegram_hosts(
    hosts: Vec<crate::models::HostInfo>,
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    let mut state_hosts = state.hosts.lock().await;
    *state_hosts = hosts;
    Ok(())
}

#[tauri::command]
pub async fn send_telegram_notification(
    title: String,
    body: String,
    host_ip: Option<String>,
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        bot.send_notification_to_all_users(&title, &body, host_ip.as_deref()).await?;
        Ok(())
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn update_telegram_user_notifications(
    user_id: i64,
    notifications_enabled: bool,
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    // Load current users from config
    let mut settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    
    // Update user notifications
    if let Some(user) = settings.telegram.registered_users.iter_mut().find(|u| u.user_id == user_id) {
        user.notifications_enabled = notifications_enabled;
    }
    
    // Save updated users to config
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    
    // Also update in bot if it's running
    let bot_guard = state.bot.lock().await;
    if let Some(ref bot) = *bot_guard {
        let _ = bot.update_user_notifications(user_id, notifications_enabled).await; // Ignore errors from bot
    }
    
    Ok(())
}

#[tauri::command]
pub async fn save_telegram_bot_token(
    token: String,
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    let mut token_guard = state.bot_token.lock().await;
    *token_guard = Some(token.clone());
    
    // Save to config file
    let mut settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.telegram.bot_token = Some(token);
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_telegram_bot_token(
    state: State<'_, TelegramBotState>,
) -> Result<Option<String>, String> {
    let token_guard = state.bot_token.lock().await;
    
    // If token is not in memory, try to load from config file
    if token_guard.is_none() {
        let settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
        if let Some(token) = settings.telegram.bot_token {
            drop(token_guard); // Release the lock
            let mut token_guard = state.bot_token.lock().await;
            *token_guard = Some(token.clone());
            return Ok(Some(token));
        }
    }
    
    Ok(token_guard.clone())
}

#[tauri::command]
pub async fn clear_telegram_bot_token(
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    let mut token_guard = state.bot_token.lock().await;
    *token_guard = None;
    
    // Remove from config file
    let mut settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.telegram.bot_token = None;
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_telegram_settings(
    state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    // Load settings from config file and populate memory
    let settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    
    if let Some(token) = settings.telegram.bot_token {
        let mut token_guard = state.bot_token.lock().await;
        *token_guard = Some(token);
    }
    
    // Load users into bot if it's running
    let bot_guard = state.bot.lock().await;
    if let Some(ref bot) = *bot_guard {
        for user in &settings.telegram.registered_users {
            let _ = bot.add_user(user.clone()).await; // Ignore errors
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_telegram_registration_info(
    state: State<'_, TelegramBotState>,
) -> Result<Option<String>, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        let reg_state = bot.get_registration_state().await;
        if reg_state.is_active && !reg_state.is_expired() {
            let remaining_attempts = reg_state.max_attempts - reg_state.attempts;
            let expires_in = if let Some(expires_at) = reg_state.expires_at {
                let now = chrono::Utc::now();
                if expires_at > now {
                    let duration = expires_at - now;
                    format!("{} minutes", duration.num_minutes())
                } else {
                    "expired".to_string()
                }
            } else {
                "unknown".to_string()
            };
            
            Ok(Some(format!(
                "Registration active. Code: {}. Attempts remaining: {}. Expires in: {}",
                reg_state.code.as_deref().unwrap_or("unknown"),
                remaining_attempts,
                expires_in
            )))
        } else {
            Ok(None)
        }
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn save_telegram_users(
    users: Vec<TelegramUser>,
    _state: State<'_, TelegramBotState>,
) -> Result<(), String> {
    // Save to config file
    let mut settings = AppSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.telegram.registered_users = users;
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}
