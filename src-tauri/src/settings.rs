use std::{
    fs,
    path::{Path, PathBuf}, sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::IgnorePoisoned;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
            match fs::read_to_string(&config_path) {
                Ok(content) => {
                    match toml::from_str::<AppSettings>(&content) {
                        Ok(settings) => {
                            // Validate settings before returning
                            if settings.is_valid() {
                                settings
                            } else {
                                eprintln!("Invalid settings found, using defaults");
                                let defaults = Self::default();
                                if let Err(e) = defaults.save() {
                                    eprintln!("Failed to save default settings: {}", e);
                                }
                                defaults
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse settings file: {}. Using defaults.", e);
                            let defaults = Self::default();
                            if let Err(e) = defaults.save() {
                                eprintln!("Failed to save default settings: {}", e);
                            }
                            defaults
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read settings file: {}. Using defaults.", e);
                    Self::default()
                }
            }
        } else {
            let defaults = Self::default();
            if let Err(e) = defaults.save() {
                eprintln!("Failed to save default settings: {}", e);
            }
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

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        
        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.timer_interval_ms > 0 
            && self.sip_amount_ml > 0 
            && self.daily_goal_ml > 0
            && self.timer_interval_ms <= 86400000 // Max 24 hours
            && self.sip_amount_ml <= 1000 // Max 1L per sip
            && self.daily_goal_ml <= 10000 // Max 10L per day
    }

    pub fn update_with_partial(&mut self,partial: PartialAppSettings ) -> anyhow::Result<()> {
        if let Some(timer_interval_ms) = partial.timer_interval_ms {
            self.timer_interval_ms = timer_interval_ms;
        }
        if let Some(sip_amount_ml) = partial.sip_amount_ml {
            self.sip_amount_ml = sip_amount_ml;
        }
        if let Some(notifications_enabled) = partial.notifications_enabled {
            self.notifications_enabled = notifications_enabled;
        }
        if let Some(start_minimized) = partial.start_minimized {
            self.start_minimized = start_minimized;
        }
        if let Some(daily_goal_ml) = partial.daily_goal_ml {
            self.daily_goal_ml = daily_goal_ml;
        }
        
        // Validate the updated settings
        if !self.is_valid() {
            return Err(anyhow::anyhow!("Invalid settings after update"));
        }

        return Ok(());   
    }
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialAppSettings {
    pub timer_interval_ms: Option<u64>,
    pub sip_amount_ml: Option<i64>,
    pub notifications_enabled: Option<bool>,
    pub start_minimized: Option<bool>,
    pub daily_goal_ml: Option<i64>,
}

#[tauri::command]
pub fn get_settings(settings: State<Mutex<AppSettings>>) -> AppSettings {
    let settings = settings.lock().ignore_poisoned();
    settings.clone()
}

#[tauri::command]
pub fn update_settings(current_settings: State<Mutex<AppSettings>>, settings: PartialAppSettings) -> Result<AppSettings, String> {
    dbg!(&current_settings);
    dbg!(&settings);

    let mut current_settings = current_settings.lock().ignore_poisoned();

    current_settings.update_with_partial(settings).map_err(|e| e.to_string())?;
    current_settings.save().map_err(|e| e.to_string())?;

  Ok(current_settings.clone())
}

#[tauri::command]
fn save_settings(settings: AppSettings) -> anyhow::Result<()> {
    settings.save()
}