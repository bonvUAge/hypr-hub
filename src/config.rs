use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
pub struct CommandItem {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Category {
    pub name: String,
    pub commands: Vec<CommandItem>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub title: String,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::find_config_file()?;

        let contents = fs::read_to_string(&config_path)
            .map_err(|e| format!("❌ Cannot read config: {}", e))?;

        let config: Config =
            toml::from_str(&contents).map_err(|e| format!("❌ Config parse error: {}", e))?;

        println!("✅ Config loaded from: {:?}", config_path);
        Ok(config)
    }

    fn find_config_file() -> Result<PathBuf, String> {
        let local_config = PathBuf::from("./src/config/config.toml");
        if local_config.exists() {
            return Ok(local_config);
        }

        if let Some(home) = dirs::home_dir() {
            let user_config = home.join(".config/hypr-hub/config.toml");
            if user_config.exists() {
                return Ok(user_config);
            }
        }

        Err("❌ config.toml not found!".to_string())
    }

    pub fn default() -> Self {
        Config {
            app: AppConfig {
                title: "Main system HUB 🦀".to_string(),
            },
            categories: vec![
                Category {
                    name: "System".to_string(),
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
                },
                Category {
                    name: "Personalization".to_string(),
                    commands: vec![CommandItem {
                        name: "🎨 Change Theme".to_string(),
                        command: "echo 'Theme changer coming soon'".to_string(),
                    }],
                },
            ],
        }
    }
}
