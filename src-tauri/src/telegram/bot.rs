use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::models::{TelegramUser, RegistrationState};
use std::path::PathBuf;
use serde_json;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "MHS Bot commands:")]
enum Command {
    #[command(description = "Get your user ID")]
    MyId,
}

#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
    is_running: Arc<AtomicBool>,
    task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    registration_state: Arc<Mutex<RegistrationState>>,
}

impl TelegramBot {
    pub async fn new(bot_token: String) -> Result<Self, String> {
        let bot = Self {
            bot: Bot::new(bot_token),
            is_running: Arc::new(AtomicBool::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
            registered_users: Arc::new(Mutex::new(Vec::new())),
            registration_state: Arc::new(Mutex::new(RegistrationState::new())),
        };
        
        // Load users from file
        bot.load_users_from_file().await?;
        
        Ok(bot)
    }

    pub async fn start(&self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err("Bot is already running".to_string());
        }

        let bot = self.bot.clone();
        let is_running = self.is_running.clone();
        let task_handle = self.task_handle.clone();

        let registered_users = self.registered_users.clone();
        let registration_state = self.registration_state.clone();
        
        let handle = tokio::spawn(async move {
            is_running.store(true, Ordering::Relaxed);
            
            let handler = dptree::entry()
                .branch(Update::filter_message().endpoint(move |bot, msg| {
                    let users = registered_users.clone();
                    let reg_state = registration_state.clone();
                    message_handler(bot, msg, users, reg_state)
                }))
                .branch(Update::filter_callback_query().endpoint(callback_handler));

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
        let mut reg_state = self.registration_state.lock().await;
        if reg_state.is_active {
            return Err("Registration is already active".to_string());
        }
        
        let code = reg_state.start_registration();
        Ok(code)
    }

    pub async fn stop_registration(&self) -> Result<(), String> {
        let mut reg_state = self.registration_state.lock().await;
        reg_state.finish_registration();
        Ok(())
    }

    pub async fn is_registration_active(&self) -> bool {
        let reg_state = self.registration_state.lock().await;
        reg_state.is_active && !reg_state.is_expired()
    }

    fn get_users_file_path() -> Result<PathBuf, String> {
        let data_dir = dirs::data_dir()
            .ok_or("Failed to get data directory")?
            .join("moonraker-host-scanner");
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
        
        Ok(data_dir.join("telegram_users.json"))
    }

    pub async fn save_users_to_file(&self) -> Result<(), String> {
        let users = self.registered_users.lock().await;
        let file_path = Self::get_users_file_path()?;
        
        let json = serde_json::to_string_pretty(&*users)
            .map_err(|e| format!("Failed to serialize users: {}", e))?;
        
        tokio::fs::write(&file_path, json).await
            .map_err(|e| format!("Failed to write users file: {}", e))?;
        
        Ok(())
    }

    pub async fn load_users_from_file(&self) -> Result<(), String> {
        let file_path = Self::get_users_file_path()?;
        
        if !file_path.exists() {
            return Ok(()); // No file to load, start with empty list
        }
        
        let content = tokio::fs::read_to_string(&file_path).await
            .map_err(|e| format!("Failed to read users file: {}", e))?;
        
        let users: Vec<TelegramUser> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse users file: {}", e))?;
        
        let mut registered_users = self.registered_users.lock().await;
        *registered_users = users;
        
        Ok(())
    }

    pub async fn get_registered_users(&self) -> Vec<TelegramUser> {
        let users = self.registered_users.lock().await;
        users.clone()
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
}

// Standalone function to save users to file
async fn save_users_to_file(users: &[TelegramUser]) -> Result<(), String> {
    let data_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("moonraker-host-scanner");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;
    
    let file_path = data_dir.join("telegram_users.json");
    
    let json = serde_json::to_string_pretty(users)
        .map_err(|e| format!("Failed to serialize users: {}", e))?;
    
    tokio::fs::write(&file_path, json).await
        .map_err(|e| format!("Failed to write users file: {}", e))?;
    
    Ok(())
}

async fn message_handler(
    bot: Bot, 
    msg: Message, 
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    registration_state: Arc<Mutex<RegistrationState>>
) -> ResponseResult<()> {
    let user_id = msg.from().unwrap().id;
    let is_registered = {
        let users = registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id.0 as i64)
    };

    if let Some(text) = msg.text() {
        // Handle commands
        if text.starts_with('/') {
            if let Ok(command) = Command::parse(text, "") {
                match command {
                    Command::MyId => {
                        if is_registered {
                            bot.send_message(msg.chat.id, format!("Your User ID: {}", user_id))
                                .await?;
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            // Handle registration code or ignore unregistered users
            if !is_registered {
                let reg_state = registration_state.lock().await;
                if reg_state.is_active && !reg_state.is_expired() {
                    if reg_state.verify_code(text) {
                        // Registration successful
                        drop(reg_state);
                        let mut reg_state = registration_state.lock().await;
                        reg_state.finish_registration();
                        
                        // Add user to registered users
                        let from_user = msg.from().unwrap();
                        let user = TelegramUser::from_teloxide_user(
                            user_id,
                            from_user.username.clone(),
                            from_user.first_name.clone(),
                            from_user.last_name.clone(),
                        );
                        
                        let mut users = registered_users.lock().await;
                        users.push(user.clone());
                        
                        bot.send_message(msg.chat.id, "Registration successful! You are now registered.")
                            .await?;
                        
                        // Save users to file
                        let users_to_save = users.clone();
                        drop(users); // Release the lock before calling save
                        if let Err(e) = save_users_to_file(&users_to_save).await {
                            println!("Failed to save users to file: {}", e);
                        }
                        
                        // Notify frontend that registration is complete
                        println!("Registration completed for user: {}", user_id.0);
                    } else {
                        bot.send_message(msg.chat.id, "Invalid code. Please try again.")
                            .await?;
                    }
                } else {
                    // Ignore unregistered users - don't send any response
                    return Ok(());
                }
            } else {
                // User is registered, can process messages
                bot.send_message(msg.chat.id, format!("Hello registered user! Your User ID: {}", user_id))
                    .await?;
            }
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;
        if let Some(msg) = q.message {
            bot.edit_message_text(msg.chat.id, msg.id, format!("You pressed: {}", data))
                .await?;
        }
    }
    Ok(())
}
