use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preferences {
    pub skip_list: Vec<String>,
}

pub struct PreferenceStore {
    path: PathBuf,
    pub preferences: Preferences,
}

impl PreferenceStore {
    pub fn new() -> Self {
        let path = Self::get_config_path();
        let mut store = Self {
            path,
            preferences: Preferences::default(),
        };
        let _ = store.load();
        store
    }

    fn get_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("str-toolkit")
            .join("preferences.json")
    }

    pub fn load(&mut self) -> Result<(), String> {
        if self.path.exists() {
            let content = fs::read_to_string(&self.path)
                .map_err(|e| format!("Failed to read preferences: {}", e))?;
            self.preferences = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse preferences: {}", e))?;
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config dir: {}", e))?;
        }
        let content = serde_json::to_string_pretty(&self.preferences)
            .map_err(|e| format!("Failed to serialize preferences: {}", e))?;
        fs::write(&self.path, content)
            .map_err(|e| format!("Failed to write preferences: {}", e))?;
        Ok(())
    }

    pub fn get_skip_list(&self) -> &Vec<String> {
        &self.preferences.skip_list
    }

    pub fn set_skip_list(&mut self, list: Vec<String>) {
        self.preferences.skip_list = list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = Preferences::default();
        assert!(prefs.skip_list.is_empty());
    }
}
