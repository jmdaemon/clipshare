mod ws;

use std::{
    fs,
    fs::read_to_string,
    path::Path,
    collections::HashMap,
};

use directories::ProjectDirs;
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};

#[macro_use] extern crate log;


const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "jmdaemon";
const APPLICATION: &str = "clipshare";

// Data Structures
type Shortcuts = HashMap<String, String>;

//pub struct UnifiedClipboard {
    //pub devices: Vec<Device>,
    //pub last_copied: String,
//}

pub struct App {
    //pub global_cb: UnifiedClipboard,
    pub current_device: Device,
    pub last_copied: String,
}

pub struct Device {
    pub name: String,
    pub history: Vec<String>,
    pub clipboard: ClipboardContext,
}

// Implementations
impl Device {
    pub fn new(name: String, history: Vec<String>, clipboard: ClipboardContext) -> Device {
        Device { name, history, clipboard }
    }

    pub fn get_clipboard_conts(&mut self) -> String {
        get_clipboard_conts(&mut self.clipboard)
    }

    pub fn set_clipboard_conts(&mut self, conts: String) {
        set_clipboard_conts(&mut self.clipboard, conts)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    max_history: u64,
    shortcuts: Shortcuts,
}

// Clipboard
fn get_clipboard_conts(ctx: &mut ClipboardContext) -> String {
    ctx.get_contents().unwrap()
}

fn set_clipboard_conts(ctx: &mut ClipboardContext, conts: String) {
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

pub fn init_device() -> Device {
    let dev_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let dev = Device::new(String::from("Device 1"), vec![String::from("asdf")], dev_ctx);
    info!("Device Name: {}", dev.name);
    info!("Device History:");
    dev.history.iter().for_each( |line| {
        info!("{}", line);
    });
    dev
}

// Project Directories
pub fn get_proj_dirs() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap()
}

pub fn mk_cfg_dir() {
    let proj_dirs = get_proj_dirs();
    let cfg_dir = proj_dirs.config_dir();
    if !cfg_dir.exists() {
        fs::create_dir(cfg_dir).expect("Could not create config file");
    }
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
