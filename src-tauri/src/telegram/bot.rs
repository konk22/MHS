use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use teloxide::{prelude::*, utils::command::BotCommands, types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, InputFile, MessageId}};
use crate::models::{TelegramUser, RegistrationState, VideoRequestState, EmergencyStopRequestState, UserSessionState, MenuState, HostCache};
use crate::models::host::HostInfo;
use crate::api::print_info::{get_print_info, format_duration};
use std::time::Duration;

/// Determines printer status based on Moonraker API flags
/// Priority order: offline > cancelling > error > paused > printing > ready > standby
fn get_printer_status(host: &HostInfo) -> String {
    // First check if host is marked as offline
    if host.status == "offline" {
        return "offline".to_string();
    }
    
    // Check if Klippy is completely disconnected (not just in error state)
    if let Some(klippy_state) = &host.klippy_state {
        if klippy_state == "disconnected" {
            return "offline".to_string();
        }
    }
    
    // If no printer flags, check if we have any device status
    if let Some(flags) = &host.printer_flags {
        // Priority order: cancelling > error > paused > printing > ready > standby
        if flags.cancelling {
            return "cancelling".to_string();
        }
        if flags.error {
            return "error".to_string();
        }
        if flags.paused {
            return "paused".to_string();
        }
        if flags.printing {
            return "printing".to_string();
        }
        if flags.ready {
            return "standby".to_string();
        }
    } else {
        // If no printer flags, check device status
        if host.device_status == "offline" || host.device_status == "klippy_disconnected" {
            return "offline".to_string();
        }
        // If Klippy is in error state but host responds, show error status
        if let Some(klippy_state) = &host.klippy_state {
            if klippy_state == "error" {
                return "error".to_string();
            }
        }
        return "standby".to_string();
    }
    
    "standby".to_string()
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "MHS Bot commands:")]
enum Command {
    #[command(description = "Start the bot and show main menu")]
    Start,
}

#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
    is_running: Arc<AtomicBool>,
    task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    _registration_state: Arc<Mutex<RegistrationState>>,
    video_request_state: Arc<Mutex<VideoRequestState>>,
    emergency_stop_request_state: Arc<Mutex<EmergencyStopRequestState>>,
    hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    http_client: reqwest::Client,
}

impl TelegramBot {
    /// Creates a new Telegram bot instance
    /// 
    /// # Arguments
    /// * `bot_token` - The Telegram bot token
    /// * `hosts` - Shared reference to the hosts list
    /// 
    /// # Returns
    /// * `Ok(TelegramBot)` - Successfully created bot instance
    /// * `Err(String)` - Error message if creation failed
    pub async fn new(bot_token: String, hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>) -> Result<Self, String> {
        // Create HTTP client with timeout configuration
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        let bot = Self {
            bot: Bot::new(bot_token),
            is_running: Arc::new(AtomicBool::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
            registered_users: Arc::new(Mutex::new(Vec::new())),
            _registration_state: Arc::new(Mutex::new(RegistrationState::new())),
            video_request_state: Arc::new(Mutex::new(VideoRequestState::new())),
            emergency_stop_request_state: Arc::new(Mutex::new(EmergencyStopRequestState::new())),
            hosts,
            user_sessions: Arc::new(Mutex::new(std::collections::HashMap::new())),
            host_cache: Arc::new(Mutex::new(HostCache::new())),
            http_client,
        };
        
        // Load users from file
        bot.load_users_from_file().await?;
        
        Ok(bot)
    }

    /// Starts the Telegram bot
    /// 
    /// # Returns
    /// * `Ok(())` - Bot started successfully
    /// * `Err(String)` - Error message if start failed
    pub async fn start(&self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err("Bot is already running".to_string());
        }

        let bot = self.bot.clone();
        let is_running = self.is_running.clone();
        let task_handle = self.task_handle.clone();

        let registered_users = self.registered_users.clone();
        let registration_state = self._registration_state.clone();
        let video_request_state = self.video_request_state.clone();
        let emergency_stop_request_state = self.emergency_stop_request_state.clone();
        let hosts = self.hosts.clone();
        let user_sessions = self.user_sessions.clone();
        let host_cache = self.host_cache.clone();
        let http_client = self.http_client.clone();
        
        let handle = tokio::spawn(async move {
            is_running.store(true, Ordering::Relaxed);
            
            // Test bot token by getting bot info first
            match bot.get_me().await {
                Ok(bot_info) => {
                    println!("Bot started successfully: @{}", bot_info.username());
                    
                    // Set bot commands menu only if bot is valid
                    if let Err(e) = bot.set_my_commands(Command::bot_commands()).await {
                        println!("Failed to set bot commands: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to start bot - invalid token: {}", e);
                    is_running.store(false, Ordering::Relaxed);
                    return;
                }
            }
            
            let handler = dptree::entry()
                .branch(Update::filter_message().endpoint({
                    let users = registered_users.clone();
                    let reg_state = registration_state.clone();
                    let video_state = video_request_state.clone();
                    let emergency_state = emergency_stop_request_state.clone();
                    let hosts = hosts.clone();
                    let sessions = user_sessions.clone();
                    let cache = host_cache.clone();
                    let client = http_client.clone();
                    move |bot, msg| {
                        message_handler(bot, msg, users.clone(), reg_state.clone(), video_state.clone(), emergency_state.clone(), hosts.clone(), sessions.clone(), cache.clone(), client.clone())
                    }
                }))
                .branch(Update::filter_callback_query().endpoint({
                    let users = registered_users.clone();
                    let sessions = user_sessions.clone();
                    let cache = host_cache.clone();
                    let hosts = hosts.clone();
                    let client = http_client.clone();
                    move |bot, q| {
                        callback_handler(bot, q, users.clone(), sessions.clone(), cache.clone(), hosts.clone(), client.clone())
                    }
                }));

            let mut dispatcher = Dispatcher::builder(bot, handler)
                .default_handler(|upd| async move {
                    println!("Unhandled update: {:?}", upd);
                })
                .build();

            dispatcher.dispatch().await;
            
            is_running.store(false, Ordering::Relaxed);
        });

        {
            let mut handle_guard = task_handle.lock().await;
            *handle_guard = Some(handle);
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Err("Bot is not running".to_string());
        }

        self.is_running.store(false, Ordering::Relaxed);
        
        let mut handle_guard = self.task_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub async fn start_registration(&self) -> Result<String, String> {
        let mut reg_state = self._registration_state.lock().await;
        if reg_state.is_active {
            return Err("Registration is already active".to_string());
        }
        
        let code = reg_state.start_registration();
        Ok(code)
    }

    pub async fn stop_registration(&self) -> Result<(), String> {
        let mut reg_state = self._registration_state.lock().await;
        reg_state.finish_registration();
        Ok(())
    }

    pub async fn is_registration_active(&self) -> bool {
        let reg_state = self._registration_state.lock().await;
        reg_state.is_active && !reg_state.is_expired()
    }

    pub async fn get_registration_state(&self) -> crate::models::RegistrationState {
        let reg_state = self._registration_state.lock().await;
        reg_state.clone()
    }


    pub async fn save_users_to_file(&self) -> Result<(), String> {
        let users = self.registered_users.lock().await;
        
        // Save to config file instead of separate file
        let mut settings = crate::models::config::AppSettings::load()
            .map_err(|e| format!("Failed to load settings: {}", e))?;
        settings.telegram.registered_users = (*users).clone();
        settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
        
        Ok(())
    }

    pub async fn load_users_from_file(&self) -> Result<(), String> {
        // Load from config file instead of separate file
        let settings = crate::models::config::AppSettings::load()
            .map_err(|e| format!("Failed to load settings: {}", e))?;
        
        let mut registered_users = self.registered_users.lock().await;
        *registered_users = settings.telegram.registered_users;
        
        Ok(())
    }

    pub async fn get_registered_users(&self) -> Vec<TelegramUser> {
        let users = self.registered_users.lock().await;
        users.clone()
    }

    pub async fn add_user(&self, user: TelegramUser) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        
        // Check if user already exists
        if users.iter().any(|u| u.user_id == user.user_id) {
            return Err("User already exists".to_string());
        }
        
        users.push(user);
        drop(users); // Release the lock before calling save
        
        // Save users to file
        self.save_users_to_file().await?;
        Ok(())
    }

    pub async fn remove_user(&self, user_id: i64) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        users.retain(|user| user.user_id != user_id);
        drop(users); // Release the lock before calling save
        
        // Save users to file
        if let Err(e) = self.save_users_to_file().await {
            println!("Failed to save users to file after removal: {}", e);
        }
        
        Ok(())
    }

    pub async fn is_user_registered(&self, user_id: teloxide::types::UserId) -> bool {
        let users = self.registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id.0 as i64)
    }

    pub async fn get_hosts(&self) -> Result<Vec<crate::models::HostInfo>, String> {
        let hosts = self.hosts.lock().await;
        Ok(hosts.clone())
    }

    pub async fn send_notification_to_all_users(&self, title: &str, body: &str, host_ip: Option<&str>) -> Result<(), String> {
        let users = self.registered_users.lock().await;
        
        if users.is_empty() {
            return Ok(()); // No users to notify
        }

        // Escape special characters for MarkdownV2
        let escaped_title = title.replace("*", "\\*").replace("_", "\\_").replace("[", "\\[").replace("]", "\\]").replace("(", "\\(").replace(")", "\\)").replace("~", "\\~").replace("`", "\\`").replace(">", "\\>").replace("#", "\\#").replace("+", "\\+").replace("-", "\\-").replace("=", "\\=").replace("|", "\\|").replace("{", "\\{").replace("}", "\\}").replace(".", "\\.").replace("!", "\\!");
        let escaped_body = body.replace("*", "\\*").replace("_", "\\_").replace("[", "\\[").replace("]", "\\]").replace("(", "\\(").replace(")", "\\)").replace("~", "\\~").replace("`", "\\`").replace(">", "\\>").replace("#", "\\#").replace("+", "\\+").replace("-", "\\-").replace("=", "\\=").replace("|", "\\|").replace("{", "\\{").replace("}", "\\}").replace(".", "\\.").replace("!", "\\!");
        
        let message = format!("🔔 *{}*\n\n{}", escaped_title, escaped_body);
        
        // Try to get webcam image if host_ip is provided
        let webcam_image = if let Some(ip) = host_ip {
            get_webcam_image(ip, &self.http_client).await.ok()
        } else {
            None
        };
        
        for user in users.iter() {
            // Only send notification if user has notifications enabled
            if !user.notifications_enabled {
                continue;
            }
            
            let result = if let Some(image_data) = &webcam_image {
                // Send message with photo
                self.bot.send_photo(teloxide::types::ChatId(user.user_id), teloxide::types::InputFile::memory(image_data.clone()))
                    .caption(&message)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await
            } else {
                // Send text message only
                self.bot.send_message(teloxide::types::ChatId(user.user_id), &message)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await
            };
            
            if let Err(e) = result {
                eprintln!("Failed to send notification to user {}: {}", user.user_id, e);
            }
        }
        
        Ok(())
    }

    pub async fn update_user_notifications(&self, user_id: i64, notifications_enabled: bool) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        
        if let Some(user) = users.iter_mut().find(|u| u.user_id == user_id) {
            user.notifications_enabled = notifications_enabled;
            drop(users); // Release the lock before calling save
            
            // Save users to file
            self.save_users_to_file().await?;
            Ok(())
        } else {
            Err(format!("User {} not found", user_id))
        }
    }
}


// Standalone function to save users to file
async fn save_users_to_file(users: &[TelegramUser]) -> Result<(), String> {
    // Save to config file instead of separate file
    let mut settings = crate::models::config::AppSettings::load()
        .map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.telegram.registered_users = users.to_vec();
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}

async fn message_handler(
    bot: Bot, 
    msg: Message, 
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    _registration_state: Arc<Mutex<RegistrationState>>,
    _video_request_state: Arc<Mutex<VideoRequestState>>,
    _emergency_stop_request_state: Arc<Mutex<EmergencyStopRequestState>>,
    _hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    _user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    _host_cache: Arc<Mutex<HostCache>>,
    _http_client: reqwest::Client
) -> ResponseResult<()> {
    let user_id = match msg.from() {
        Some(user) => user.id,
        None => return Ok(()), // Ignore messages without sender
    };
    let is_registered = {
        let users = registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id.0 as i64)
    };

    if let Some(text) = msg.text() {
        // Handle commands
        if text.starts_with('/') {
            if let Ok(command) = Command::parse(text, "") {
                match command {
                    Command::Start => {
                        if is_registered {
                            // Show main menu for registered users
                            let keyboard = InlineKeyboardMarkup::new(vec![
                                vec![InlineKeyboardButton::callback("📋 Список хостов", "hosts_list")],
                                vec![InlineKeyboardButton::callback("⚙️ Настройки", "settings")],
                                vec![InlineKeyboardButton::callback("❓ Помощь", "help")],
                            ]);

                            bot.send_message(msg.chat.id, "🤖 *Добро пожаловать в MHS Bot\\!*\n\nВыберите действие:")
                                .parse_mode(ParseMode::MarkdownV2)
                                .reply_markup(keyboard)
                                .await?;
                        } else {
                            // Ignore unregistered users - don't send any response
                            // This prevents unauthorized access and code generation
                            return Ok(());
                        }
                    }
                }
            } else {
                if is_registered {
                    bot.send_message(msg.chat.id, "❓ Неизвестная команда\\. Используйте /start для открытия главного меню\\.")
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;
                } else {
                    // Ignore unregistered users - don't send any response
                    return Ok(());
                }
            }
        } else {
            // Handle text messages
            if !is_registered {
                // Check if registration is active and user is trying to register
                let mut reg_state = _registration_state.lock().await;
                if reg_state.is_active && !reg_state.is_expired() {
                    if reg_state.verify_code(text) {
                        // Registration successful
                        reg_state.finish_registration();
                        
                        // Add user to registered users
                        let from_user = match msg.from() {
                            Some(user) => user,
                            None => return Ok(()), // Ignore messages without sender
                        };
                        let user = crate::models::TelegramUser::from_teloxide_user(
                            user_id,
                            from_user.username.clone(),
                            from_user.first_name.clone(),
                            from_user.last_name.clone(),
                        );
                        
                        // Add user to registered users
                        let mut users = registered_users.lock().await;
                        users.push(user.clone());
                        drop(users); // Release the lock
                        
                        // Show main menu after successful registration
                        let keyboard = InlineKeyboardMarkup::new(vec![
                            vec![InlineKeyboardButton::callback("📋 Список хостов", "hosts_list")],
                            vec![InlineKeyboardButton::callback("⚙️ Настройки", "settings")],
                            vec![InlineKeyboardButton::callback("❓ Помощь", "help")],
                        ]);

                        let welcome_message = format!("✅ Регистрация успешна! Добро пожаловать, {}! Выберите действие:", user.display_name());
                        bot.send_message(msg.chat.id, welcome_message)
                            .reply_markup(keyboard)
                            .await?;
                        
                        // Save users to file
                        let users_to_save = registered_users.lock().await.clone();
                        if let Err(e) = save_users_to_file(&users_to_save).await {
                            println!("Failed to save users to file: {}", e);
                        }
                        
                        // Notify frontend that registration is complete
                        println!("Registration completed for user: {}", user_id.0);
                    } else {
                        // Check if max attempts reached
                        if reg_state.attempts >= reg_state.max_attempts {
                            reg_state.finish_registration();
                            bot.send_message(msg.chat.id, "❌ Слишком много неудачных попыток\\. Регистрация отменена\\.")
                                .parse_mode(ParseMode::MarkdownV2)
                                .await?;
                        } else {
                            let remaining = reg_state.max_attempts - reg_state.attempts;
                            bot.send_message(msg.chat.id, format!("❌ Неверный код\\. Осталось попыток: {}", remaining))
                                .await?;
                        }
                    }
                } else {
                    // Registration not active or expired, ignore
                    return Ok(());
                }
            } else {
                // Registered user sent text message, show main menu
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("📋 Список хостов", "hosts_list")],
                    vec![InlineKeyboardButton::callback("⚙️ Настройки", "settings")],
                    vec![InlineKeyboardButton::callback("❓ Помощь", "help")],
                ]);

                bot.send_message(msg.chat.id, "🤖 *Главное меню*\n\nВыберите действие:")
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    }
    Ok(())
}

/// Validates IP address to prevent SSRF attacks
/// Only allows private network ranges and localhost
fn is_valid_ip_address(ip: &str) -> bool {
    use std::net::IpAddr;
    
    let ip_addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(_) => return false,
    };
    
    match ip_addr {
        IpAddr::V4(ipv4) => {
            // Allow localhost
            if ipv4.is_loopback() {
                return true;
            }
            
            // Allow private network ranges
            if ipv4.is_private() {
                return true;
            }
            
            // Allow link-local addresses
            if ipv4.is_link_local() {
                return true;
            }
            
            false
        }
        IpAddr::V6(ipv6) => {
            // Allow localhost
            if ipv6.is_loopback() {
                return true;
            }
            
            // Allow unique local addresses (fc00::/7)
            if ipv6.is_unique_local() {
                return true;
            }
            
            // Allow link-local addresses (fe80::/10)
            if ipv6.is_unicast_link_local() {
                return true;
            }
            
            false
        }
    }
}

async fn get_webcam_image(ip_address: &str, client: &reqwest::Client) -> Result<Vec<u8>, String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/webcam/?action=snapshot", ip_address);
    
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to request image: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let image_data = response.bytes()
        .await
        .map_err(|e| format!("Failed to read image data: {}", e))?
        .to_vec();
    
    Ok(image_data)
}

async fn send_emergency_stop(ip_address: &str, client: &reqwest::Client) -> Result<(), String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/printer/emergency_stop", ip_address);
    
    let response = client.post(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send emergency stop request: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    Ok(())
}

async fn send_stop_print(ip_address: &str, client: &reqwest::Client) -> Result<(), String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/printer/print/cancel", ip_address);
    
    let response = client
        .post(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    Ok(())
}

async fn send_firmware_restart(ip_address: &str, client: &reqwest::Client) -> Result<(), String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/printer/firmware_restart", ip_address);
    
    let response = client
        .post(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    Ok(())
}

async fn callback_handler(
    bot: Bot, 
    q: CallbackQuery,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    http_client: reqwest::Client,
) -> ResponseResult<()> {
    let user_id = match q.from.id.0 {
        id if id > 0 => id as i64,
        _ => return Ok(()),
    };

    // Check if user is registered
    let is_registered = {
        let users = registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id)
    };

    if !is_registered {
        // Ignore callback queries from unregistered users
        return Ok(());
    }

    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;
        
        if let Some(msg) = q.message {
            match data.as_str() {
                "main_menu" => {
                    show_main_menu(&bot, msg.chat.id, msg.id, user_sessions.clone(), user_id).await?;
                }
                "hosts_list" => {
                    show_hosts_list(&bot, msg.chat.id, msg.id, user_sessions.clone(), host_cache.clone(), hosts.clone(), user_id).await?;
                }
                "settings" => {
                    show_settings(&bot, msg.chat.id, msg.id, user_sessions.clone(), registered_users.clone(), user_id).await?;
                }
                "help" => {
                    show_help(&bot, msg.chat.id, msg.id, user_sessions.clone(), user_id).await?;
                }
                _ if data.starts_with("host_image_") => {
                    let host_id = data.strip_prefix("host_image_").unwrap_or("");
                    get_host_image(&bot, msg.chat.id, msg.id, host_cache.clone(), http_client.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("host_emergency_") => {
                    let host_id = data.strip_prefix("host_emergency_").unwrap_or("");
                    show_emergency_confirm(&bot, msg.chat.id, msg.id, user_sessions.clone(), host_cache.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("host_stop_print_") => {
                    let host_id = data.strip_prefix("host_stop_print_").unwrap_or("");
                    show_stop_print_confirm(&bot, msg.chat.id, msg.id, user_sessions.clone(), host_cache.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("host_firmware_restart_") => {
                    let host_id = data.strip_prefix("host_firmware_restart_").unwrap_or("");
                    show_firmware_restart_confirm(&bot, msg.chat.id, msg.id, user_sessions.clone(), host_cache.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("host_") => {
                    let host_id = data.strip_prefix("host_").unwrap_or("");
                    show_host_details(&bot, msg.chat.id, msg.id, user_sessions.clone(), host_cache.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("emergency_confirm_") => {
                    let host_id = data.strip_prefix("emergency_confirm_").unwrap_or("");
                    execute_emergency_stop(&bot, msg.chat.id, msg.id, host_cache.clone(), http_client.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("stop_print_confirm_") => {
                    let host_id = data.strip_prefix("stop_print_confirm_").unwrap_or("");
                    execute_stop_print(&bot, msg.chat.id, msg.id, host_cache.clone(), http_client.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("firmware_restart_confirm_") => {
                    let host_id = data.strip_prefix("firmware_restart_confirm_").unwrap_or("");
                    execute_firmware_restart(&bot, msg.chat.id, msg.id, host_cache.clone(), http_client.clone(), host_id, user_id).await?;
                }
                _ if data.starts_with("toggle_notifications_") => {
                    let action = data.strip_prefix("toggle_notifications_").unwrap_or("");
                    toggle_notifications(&bot, msg.chat.id, msg.id, registered_users.clone(), action, user_id).await?;
                }
                _ => {
                    bot.edit_message_text(msg.chat.id, msg.id, "❌ Unknown action")
                        .await?;
                }
            }
        }
    }
    Ok(())
}

async fn show_main_menu(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::Main);
    session.set_message_id(message_id);
    drop(sessions);

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("📋 Список хостов", "hosts_list")],
        vec![InlineKeyboardButton::callback("⚙️ Настройки", "settings")],
        vec![InlineKeyboardButton::callback("❓ Помощь", "help")],
    ]);

    bot.edit_message_text(chat_id, message_id, "🤖 *Главное меню*\n\nВыберите действие:")
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn show_hosts_list(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::Hosts);
    session.set_message_id(message_id);
    drop(sessions);

    // Get hosts from cache or update if stale
    let hosts_data = {
        let mut cache = host_cache.lock().await;
        if cache.is_stale() || cache.hosts.is_empty() {
            // Get hosts from the main application
            let hosts_guard = hosts.lock().await;
            let hosts_data = hosts_guard.clone();
            drop(hosts_guard);
            cache.update_hosts(hosts_data.clone());
            hosts_data
        } else {
            cache.hosts.clone()
        }
    };

    if hosts_data.is_empty() {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("🔄 Обновить", "hosts_list")],
            vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
        ]);

        bot.edit_message_text(chat_id, message_id, "📋 *Список хостов*\n\n❌ Хосты не найдены\\. Убедитесь, что приложение запущено и выполнило сканирование\\.")
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    } else {
        let mut keyboard_buttons = Vec::new();
        
        for host in &hosts_data {
            let printer_status = get_printer_status(host);
            let status_emoji = match printer_status.as_str() {
                "printing" => "🟡",
                "paused" => "⏸️",
                "error" => "❌",
                "cancelling" => "⏹️",
                "standby" => "🟢",
                "offline" => "🔴",
                _ => "⚪"
            };
            
            let button_text = format!("{} {} ({})", status_emoji, host.hostname, host.ip_address);
            keyboard_buttons.push(vec![InlineKeyboardButton::callback(button_text, format!("host_{}", host.ip_address))]);
        }
        
        keyboard_buttons.push(vec![InlineKeyboardButton::callback("🔄 Обновить", "hosts_list")]);
        keyboard_buttons.push(vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")]);
        
        let keyboard = InlineKeyboardMarkup::new(keyboard_buttons);

        bot.edit_message_text(chat_id, message_id, "📋 *Список хостов*\n\nВыберите хост для управления:")
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    }

    Ok(())
}

async fn show_host_details(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    host_id: &str,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::HostDetails(host_id.to_string()));
    session.set_message_id(message_id);
    session.selected_host_id = Some(host_id.to_string());
    drop(sessions);

    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        let printer_status = get_printer_status(&host);
        let status_emoji = match printer_status.as_str() {
            "printing" => "🟡",
            "paused" => "⏸️",
            "error" => "❌",
            "cancelling" => "⏹️",
            "standby" => "🟢",
            "offline" => "🔴",
            _ => "⚪"
        };

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("📷 Изображение", format!("host_image_{}", host_id))],
            vec![InlineKeyboardButton::callback("⏹️ Остановить печать", format!("host_stop_print_{}", host_id))],
            vec![InlineKeyboardButton::callback("🔄 Firmware Restart", format!("host_firmware_restart_{}", host_id))],
            vec![InlineKeyboardButton::callback("🛑 Экстренная остановка", format!("host_emergency_{}", host_id))],
            vec![InlineKeyboardButton::url("🌐 Открыть в браузере", format!("http://{}", host.ip_address).parse().unwrap())],
            vec![InlineKeyboardButton::callback("🔙 Назад к списку", "hosts_list")],
            vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
        ]);

        // Get print information if printer is printing or paused
        let mut print_info_text = String::new();
        if printer_status == "printing" || printer_status == "paused" {
            // Try to get print info with timeout
            let print_info_result = tokio::time::timeout(
                Duration::from_secs(3),
                get_print_info(&host.ip_address, None)
            ).await;
            
            match print_info_result {
                Ok(Ok(Some(print_job))) => {
                    let progress = print_job.progress.progress;
                    let print_duration = format_duration(print_job.progress.print_duration);
                    let remaining_time = if print_job.progress.total_duration > print_job.progress.print_duration {
                        format_duration(print_job.progress.total_duration - print_job.progress.print_duration)
                    } else {
                        "Неизвестно".to_string()
                    };
                    
                    // Use filename as-is without escaping
                    print_info_text = format!(
                        "\n🖨️ {}\n📈 {:.1}% | ⏱️ {} | ⏳ {}",
                        print_job.filename, progress, print_duration, remaining_time
                    );
                }
                _ => {
                    print_info_text = "\n🖨️ Информация о печати недоступна".to_string();
                }
            }
        }

        let message = format!(
            "🖥️ {}\n\n{} IP: {}\n📊 Статус: {}{}\n\nВыберите действие:",
            host.hostname,
            status_emoji,
            host.ip_address,
            printer_status,
            print_info_text
        );

        bot.edit_message_text(chat_id, message_id, message)
            .reply_markup(keyboard)
            .await?;
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }
    Ok(())
}

async fn show_emergency_confirm(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    host_id: &str,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::EmergencyConfirm(host_id.to_string()));
    session.set_message_id(message_id);
    session.emergency_confirmation = true;
    drop(sessions);

    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("✅ ПОДТВЕРДИТЬ ОСТАНОВКУ", format!("emergency_confirm_{}", host_id))],
            vec![InlineKeyboardButton::callback("❌ Отмена", format!("host_{}", host_id))],
            vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
        ]);

        let message = format!(
            "⚠️ *ЭКСТРЕННАЯ ОСТАНОВКА*\n\n🖥️ Хост: {}\n📍 IP: `{}`\n\n🚨 **ВНИМАНИЕ:** Это действие немедленно остановит принтер\\!\n\nВы уверены, что хотите продолжить\\?",
            host.hostname,
            host.ip_address
        );

        bot.edit_message_text(chat_id, message_id, message)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn show_stop_print_confirm(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    host_id: &str,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::EmergencyConfirm(host_id.to_string()));
    session.set_message_id(message_id);
    drop(sessions);

    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("✅ Да, остановить печать", format!("stop_print_confirm_{}", host_id))],
            vec![InlineKeyboardButton::callback("❌ Отмена", format!("host_{}", host_id))],
        ]);

        bot.edit_message_text(chat_id, message_id, format!("⚠️ Вы уверены, что хотите остановить печать на {}?\n\nЭто действие нельзя отменить.", host.hostname))
            .reply_markup(keyboard)
            .await?;
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn show_firmware_restart_confirm(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    host_cache: Arc<Mutex<HostCache>>,
    host_id: &str,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::EmergencyConfirm(host_id.to_string()));
    session.set_message_id(message_id);
    drop(sessions);

    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("✅ Да, перезагрузить firmware", format!("firmware_restart_confirm_{}", host_id))],
            vec![InlineKeyboardButton::callback("❌ Отмена", format!("host_{}", host_id))],
        ]);

        bot.edit_message_text(chat_id, message_id, format!("⚠️ Вы уверены, что хотите перезагрузить firmware на {}?\n\nПринтер будет перезагружен и может быть недоступен несколько секунд.", host.hostname))
            .reply_markup(keyboard)
            .await?;
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn execute_emergency_stop(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    host_cache: Arc<Mutex<HostCache>>,
    http_client: reqwest::Client,
    host_id: &str,
    _user_id: i64,
) -> ResponseResult<()> {
    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        bot.edit_message_text(chat_id, message_id, format!("🛑 Отправка экстренной остановки на {}...", host.hostname))
            .await?;

        // Send emergency stop command
        match send_emergency_stop(&host.ip_address, &http_client).await {
            Ok(_) => {
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("🔙 Назад к хосту", format!("host_{}", host_id))],
                    vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
                ]);

                bot.edit_message_text(chat_id, message_id, format!("✅ Экстренная остановка успешно отправлена на {}!", host.hostname))
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(e) => {
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("🔙 Назад к хосту", format!("host_{}", host_id))],
                    vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
                ]);

                bot.edit_message_text(chat_id, message_id, format!("❌ Ошибка отправки экстренной остановки: {}", e))
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn execute_stop_print(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    host_cache: Arc<Mutex<HostCache>>,
    http_client: reqwest::Client,
    host_id: &str,
    _user_id: i64,
) -> ResponseResult<()> {
    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        bot.edit_message_text(chat_id, message_id, format!("⏹️ Остановка печати на {}...", host.hostname))
            .await?;

        // Send stop print request
        match send_stop_print(&host.ip_address, &http_client).await {
            Ok(_) => {
                bot.edit_message_text(chat_id, message_id, format!("✅ Печать остановлена на {}", host.hostname))
                    .await?;
            }
            Err(e) => {
                bot.edit_message_text(chat_id, message_id, format!("❌ Ошибка остановки печати на {}: {}", host.hostname, e))
                    .await?;
            }
        }
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn execute_firmware_restart(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    host_cache: Arc<Mutex<HostCache>>,
    http_client: reqwest::Client,
    host_id: &str,
    _user_id: i64,
) -> ResponseResult<()> {
    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        bot.edit_message_text(chat_id, message_id, format!("🔄 Перезагрузка firmware на {}...", host.hostname))
            .await?;

        // Send firmware restart request
        match send_firmware_restart(&host.ip_address, &http_client).await {
            Ok(_) => {
                bot.edit_message_text(chat_id, message_id, format!("✅ Firmware перезагружен на {}", host.hostname))
                    .await?;
            }
            Err(e) => {
                bot.edit_message_text(chat_id, message_id, format!("❌ Ошибка перезагрузки firmware на {}: {}", host.hostname, e))
                    .await?;
            }
        }
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn get_host_image(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    host_cache: Arc<Mutex<HostCache>>,
    http_client: reqwest::Client,
    host_id: &str,
    _user_id: i64,
) -> ResponseResult<()> {
    // Find host in cache
    let host = {
        let cache = host_cache.lock().await;
        cache.hosts.iter().find(|h| h.ip_address == host_id).cloned()
    };

    if let Some(host) = host {
        bot.edit_message_text(chat_id, message_id, format!("📷 Получение изображения с {}...", host.hostname))
            .await?;

        // Get image from webcam
        match get_webcam_image(&host.ip_address, &http_client).await {
            Ok(image_data) => {
                // Send image to user
                bot.send_photo(chat_id, InputFile::memory(image_data))
                    .caption(format!("📷 Изображение с {}", host.hostname))
                    .await?;

                // Update the message with navigation buttons
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("🔙 Назад к хосту", format!("host_{}", host_id))],
                    vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
                ]);

                bot.edit_message_text(chat_id, message_id, "✅ Изображение получено!")
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(e) => {
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("🔙 Назад к хосту", format!("host_{}", host_id))],
                    vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
                ]);

                bot.edit_message_text(chat_id, message_id, format!("❌ Ошибка получения изображения: {}", e))
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Хост не найден")
            .await?;
    }

    Ok(())
}

async fn show_settings(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::Settings);
    session.set_message_id(message_id);
    drop(sessions);

    // Get user notification settings
    let notifications_enabled = {
        let users = registered_users.lock().await;
        users.iter().find(|u| u.user_id == user_id)
            .map(|u| u.notifications_enabled)
            .unwrap_or(false)
    };

    let notification_text = if notifications_enabled {
        "🔔 Включены"
    } else {
        "🔕 Выключены"
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            format!("{} Уведомления", notification_text),
            if notifications_enabled { "toggle_notifications_off" } else { "toggle_notifications_on" }
        )],
        vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
    ]);

    bot.edit_message_text(chat_id, message_id, format!("⚙️ *Настройки*\n\n🔔 Уведомления: {}", notification_text))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn show_help(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    user_sessions: Arc<Mutex<std::collections::HashMap<i64, UserSessionState>>>,
    user_id: i64,
) -> ResponseResult<()> {
    let mut sessions = user_sessions.lock().await;
    let session = sessions.entry(user_id).or_insert_with(|| UserSessionState::new(user_id));
    session.set_menu(MenuState::Main);
    session.set_message_id(message_id);
    drop(sessions);

    let help_text = "❓ Помощь\n\n\
🤖 MHS Bot - бот для мониторинга 3D принтеров\n\n\
📋 Основные функции:\n\
• Просмотр списка хостов\n\
• Мониторинг статуса принтеров\n\
• Получение изображений с камер\n\
• Экстренная остановка печати\n\
• Открытие веб-интерфейса\n\n\
⚙️ Настройки:\n\
• Управление уведомлениями\n\n\
🔧 Поддержка:\n\
Обратитесь к администратору";

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
    ]);

    bot.edit_message_text(chat_id, message_id, help_text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn toggle_notifications(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    action: &str,
    user_id: i64,
) -> ResponseResult<()> {
    let enable = action == "on";
    
    let mut users = registered_users.lock().await;
    if let Some(user) = users.iter_mut().find(|u| u.user_id == user_id) {
        user.notifications_enabled = enable;
        drop(users); // Release the lock before calling save
        
        // Save users to file
        let users_to_save = registered_users.lock().await.clone();
        if let Err(e) = save_users_to_file(&users_to_save).await {
            println!("Failed to save users to file: {}", e);
        }
        
        let status_text = if enable { "включены" } else { "выключены" };
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                format!("{} Уведомления", if enable { "🔔 Включены" } else { "🔕 Выключены" }),
                if enable { "toggle_notifications_off" } else { "toggle_notifications_on" }
            )],
            vec![InlineKeyboardButton::callback("🏠 Главное меню", "main_menu")],
        ]);

        bot.edit_message_text(chat_id, message_id, format!("✅ Уведомления {}!", status_text))
            .reply_markup(keyboard)
            .await?;
    } else {
        bot.edit_message_text(chat_id, message_id, "❌ Пользователь не найден")
            .await?;
    }

    Ok(())
}
