// Config.toml

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
pub struct CommandItem {
    pub name: String,    // Название команды в меню
    pub command: String, // Сама команда для выполнения
}

// 🦀 Главная структура конфига
#[derive(Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
    pub commands: Vec<CommandItem>,
}

// 🦀 Настройки приложения
#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub title: String,
}

impl Config {
    // 🦀 Загружаем конфиг из файла
    pub fn load() -> Result<Self, String> {
        // Ищем файл конфига
        let config_path = Self::find_config_file()?;

        // Читаем содержимое файла
        let contents =
            fs::read_to_string(&config_path).map_err(|e| format!("❌ cant read config: {}", e))?;

        // Парсим TOML в структуру Config
        let config: Config =
            toml::from_str(&contents).map_err(|e| format!("❌ parse config error: {}", e))?;

        println!("✅ Config loaded from: {:?}", config_path);
        Ok(config)
    }

    // 🦀 Ищем файл конфига в разных местах
    fn find_config_file() -> Result<PathBuf, String> {
        // 1. Сначала ищем в текущей директории
        let local_config = PathBuf::from("config.toml");
        if local_config.exists() {
            return Ok(local_config);
        }

        // 2. Потом в ~/.config/hypr-hub/config.toml
        if let Some(home) = dirs::home_dir() {
            let user_config = home.join(".config/hypr-hub/config.toml");
            if user_config.exists() {
                return Ok(user_config);
            }
        }

        Err("❌ File config.toml not found! Create it in the current directory or in ~/.config/hypr-hub/".to_string())
    }

    // 🦀 Default config
    pub fn default() -> Self {
        Config {
            app: AppConfig {
                title: "Main system HUB 🦀".to_string(),
            },
            commands: vec![
                CommandItem {
                    name: "🔂 Update System".to_string(),
                    command: "./scripts/update.sh".to_string(),
                },
                CommandItem {
                    name: "🧹 Clean Pacman&Paru Cache".to_string(),
                    command: "./scripts/scc.sh".to_string(),
                },
                CommandItem {
                    name: "🪠 Clean RAM".to_string(),
                    command: "sudo sync; sudo sysctl -w vm.drop_caches=3".to_string(),
                },
            ],
        }
    }
}
