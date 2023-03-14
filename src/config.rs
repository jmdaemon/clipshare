use crate::consts::{QUALIFIER, ORGANIZATION, APPLICATION};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
    collections::HashMap,
};

use derivative::Derivative;
use directories::ProjectDirs;

/// Stores the path of an application config file
#[derive(Derivative)]
#[derivative(Debug, Default, Clone)]
pub struct Config {
    #[derivative(Default(value = "ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).expect(\"Could not initialize config\")"))]
    pub project_dirs: ProjectDirs,
    pub path: PathBuf,
}

impl Config {
    pub fn new(file: impl Into<String>) -> Config {
        let cfg = Config::default();
        Config {path: cfg.format_path(file.into()), ..cfg}
    }

    fn format_path(&self, config: impl Into<String>) -> PathBuf {
        self.project_dirs.config_dir().to_path_buf().join(config.into())
    }

    pub fn make_dirs(&self) {
        create_dir_all(self.project_dirs.config_dir()).expect("Could not create config directory")
    }

    pub fn read(&self) -> String {
        read_to_string(&self.path).expect("Could not read config file.")
    }

    pub fn write(&self, conts: &str) -> std::io::Result<()> {
        write(&self.path, conts)
    }
}

/*
pub struct ConfigFile<T> {
    pub config: Config,
    _marker: PhantomData<T>,
}

// Functions to use when serializing/deserializing
pub trait SerdeDispatch<'de>: Sized {
    fn serialize() -> String;
    fn deserialize<D>(cfg: D) -> String
        where D: Deserialize<'de>;
        //where D: Deserializer<'de>;
}

pub struct Json;

impl <'de> SerdeDispatch<'de> for Json {
    fn deserialize(cfg: Box<impl Serialize>) -> String {
        serde_json::to_string(&cfg).unwrap()
    }
    
    //fn deserialize<D>(cfg: D) -> String
    fn serialize<D>(cfg: D) -> String
    where
        //D: Deserialize<'de>
        D: Deserializer<'de>
    {
        //serde_json::to_string(&cfg).unwrap()
        //serde_json::to_string(&cfg).unwrap()
        //serde_json::to_string_pretty(&cfg).unwrap()
        serde_json::from_str(&cfg).unwrap()
    }
}

//pub enum FileType {
    //JSON
//}

*/


use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize, Deserializer};

type Shortcuts = HashMap<String, String>;

// Clipboard
pub fn get_clipboard_conts(ctx: &mut ClipboardContext) -> String {
    ctx.get_contents().unwrap()
}

pub fn set_clipboard_conts(ctx: &mut ClipboardContext, conts: String) {
    ctx.set_contents(conts).expect("Could not set contents of clipboard");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    max_history: u64,
    shortcuts: Shortcuts,
    #[serde(skip)]
    pub config: Config,
}

impl Default for Settings {
    fn default() -> Self {
        let max_history = 10_000;
        let shortcuts = HashMap::from([
            ("Enable/Disable Device".to_owned(), "Ctrl + {}".to_owned()),
        ]);
        let config = Config::new("config.json");
        Settings { max_history, shortcuts, config }
    }
}

impl Settings {
    pub fn new(&self, max_history: u64, shortcuts: Shortcuts, config: Config) -> Self {
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
