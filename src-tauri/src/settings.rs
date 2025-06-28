use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub timer_interval_ms: u64,
    pub sip_amount_ml: i64,
    pub notifications_enabled: bool,
    pub start_minimized: bool,
    pub daily_goal_ml: i64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            timer_interval_ms: 1000,
            sip_amount_ml: 35,
            notifications_enabled: true,
            start_minimized: true,
            daily_goal_ml: 2000,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => toml::from_str(&content).unwrap_or_default(),
                Err(e) => {
                    eprintln!("Failed to read settings file: {}. Using defaults.", e);
                    Self::default()
                }
            }
        } else {
            let defaults = Self::default();
            defaults.save();
            defaults
        }
    }

    fn config_path() -> PathBuf {
        match dirs::config_local_dir() {
            Some(dir) => dir.join("waterer").join("settings.toml"),
            None => Path::new(".")
                .join("waterer")
                .join("settings.toml")
                .to_path_buf(),
        }
    }

    fn save(&self) {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&config_path, content) {
                    eprintln!("Failed to save settings: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize settings: {}", e);
            }
        }
    }
}
