use crate::{config::ConfigFile, create_config};

use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
};

use serde::{Serialize, Deserialize};

type Shortcuts = HashMap<String, String>;

/// Store application settings
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    max_history: u64,
    shortcuts: Shortcuts,
    #[serde(skip)]
    pub config: ConfigFile,
}

impl Default for Settings {
    fn default() -> Self {
        let max_history = 10_000;
        let shortcuts = HashMap::from([
            ("Enable/Disable Device".to_owned(), "Ctrl + {}".to_owned()),
        ]);
        let config = create_config("config.json");
        Settings { max_history, shortcuts, config }
    }
}

impl Settings {
    pub fn new(&self, max_history: u64, shortcuts: Shortcuts, config: ConfigFile) -> Self {
        Settings { max_history, shortcuts, config }
    }

    pub fn load_config(&mut self) -> Self {
        //let file = self.config.format_path(self.config.path.to_str().unwrap());
        let file = &self.config.path;
        let settings: Settings = if file.exists() {
            // Read config file into memory
            let conts = read_to_string(file).expect("Could not read config file");
            let mut settings: Settings = serde_json::from_str(&conts).unwrap();
            settings.config = self.config.to_owned();
            settings
        } else {
            // Load the default settings
            Settings::default()
        };
        settings
    }

    pub fn save(&self) {
        self.config.make_dirs();
        let conts = serde_json::to_string_pretty(&self).unwrap();
        self.config.write(&conts).expect("Could not save settings to disk");
    }
}

/// Builder to construct Settings more easily
#[derive(Default)]
pub struct SettingsBuilder {
    pub settings: Settings,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        SettingsBuilder { settings: Settings::default() }
    }

    pub fn build(self) -> Settings {
        self.settings
    }

    pub fn max_history(mut self, max_history: u64) -> Self {
        self.settings.max_history = max_history;
        self
    }

    pub fn shortcuts(mut self, shortcuts: Shortcuts) -> Self {
        self.settings.shortcuts = shortcuts;
        self
    }

    pub fn config(mut self, config: ConfigFile) -> Self {
        self.settings.config = config;
        self
    }
}
