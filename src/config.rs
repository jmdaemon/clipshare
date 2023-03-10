use crate::consts::{QUALIFIER, ORGANIZATION, APPLICATION};
use std::{
    fs,
    fs::read_to_string,
    path::Path,
    collections::HashMap,
};

use directories::ProjectDirs;
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};

type Shortcuts = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    max_history: u64,
    shortcuts: Shortcuts,
}

// Clipboard
pub fn get_clipboard_conts(ctx: &mut ClipboardContext) -> String {
    ctx.get_contents().unwrap()
}

pub fn set_clipboard_conts(ctx: &mut ClipboardContext, conts: String) {
    ctx.set_contents(conts).expect("Could not set contents of clipboard");
}

impl Config {
    pub fn default() -> Config {
        let shortcuts = HashMap::from([
                (String::from("Enable/Disable Device"), String::from("Ctrl + {}")),
        ]);
        Config { max_history:10_000, shortcuts}
    }
    pub fn new(&self, max_history: u64, shortcuts: Shortcuts) -> Config {
        Config { max_history, shortcuts }
    }
}

// Project Directories
pub fn get_proj_dirs() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap()
}

pub fn mk_cfg_dir() {
    let proj_dirs = get_proj_dirs();
    let cfg_dir = proj_dirs.config_dir();
    if !cfg_dir.exists() {
        fs::create_dir_all(cfg_dir).expect("Could not create config file");
        info!("Created {}", cfg_dir.display());
    }
    info!("Using config directory: {}", cfg_dir.display());
}

pub fn get_cfgfp() -> String {
    let proj_dirs = get_proj_dirs();
    format!("{}/{}", proj_dirs.config_dir().to_str().unwrap(), "config.json")
}

/// Loads the cached config file
pub fn load_config() -> Config {
    let cfgfp = get_cfgfp();
    let cfgfp = Path::new(&cfgfp);

    let cfg: Config;
    let cfg_conts: String;
    if !cfgfp.exists() {
        cfg = Config::default();
        cfg_conts = serde_json::to_string(&cfg).unwrap();
        save_config(&cfg);
    } else {
        cfg_conts = read_to_string(cfgfp).expect("Could not read config file");
        cfg = serde_json::from_str(&cfg_conts).unwrap();
    }
    info!("Config:\n{}", cfg_conts);
    cfg
}

/// Save the config to disk
pub fn save_config(cfg: &Config) {
    let cfgfp = get_cfgfp();
    let cfg_json = serde_json::to_string_pretty(&cfg).unwrap();
    mk_cfg_dir();
    fs::write(cfgfp, cfg_json).expect("Unable to write file.");
}