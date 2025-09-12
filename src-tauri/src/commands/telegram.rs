use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::telegram::TelegramBot;
use crate::models::TelegramUser;

pub struct TelegramBotState {
    pub bot: Arc<Mutex<Option<TelegramBot>>>,
}

impl TelegramBotState {
    pub fn new() -> Self {
        Self {
            bot: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn start_telegram_bot(
    bot_token: String,
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let mut bot_guard = state.bot.lock().await;
    
    // Stop existing bot if running
    if let Some(ref bot) = *bot_guard {
        if bot.is_running() {
            let _ = bot.stop().await;
        }
    }
    
    // Create and start new bot
    let bot = TelegramBot::new(bot_token).await?;
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
    state: State<'_, TelegramBotState>,
) -> Result<Vec<TelegramUser>, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        Ok(bot.get_registered_users().await)
    } else {
        Err("Bot is not running".to_string())
    }
}

#[tauri::command]
pub async fn remove_telegram_user(
    user_id: i64,
    state: State<'_, TelegramBotState>,
) -> Result<String, String> {
    let bot_guard = state.bot.lock().await;
    
    if let Some(ref bot) = *bot_guard {
        bot.remove_user(user_id).await?;
        Ok("User removed successfully".to_string())
    } else {
        Err("Bot is not running".to_string())
    }
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
