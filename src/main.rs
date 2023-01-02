// Standard Library
use std::collections::HashMap;

// Third Party Libraries
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// Data Structures
pub struct Device {
    name: String,
    history: Vec<String>,
    clipboard: ClipboardContext,
}

impl Device {
    pub fn new(name: String, history: Vec<String>, clipboard: ClipboardContext) -> Device {
        Device { name, history, clipboard }
    }
}

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

fn main() {
    pretty_env_logger::init();

    let dev_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let dev = Device::new(String::from("Device 1"), vec![String::from("asdf")], dev_ctx);
    info!("Device Name: {}", dev.name);
    info!("Device History:");
    dev.history.iter().for_each( |line| {
        info!("{}", line);
    });

    let cfg = Config::default();
    let cfg_json = serde_json::to_string(&cfg).unwrap();
    info!("Config:\n{}", cfg_json);


    let mut ctx = dev.clipboard;
    println!("{}", get_clipboard_conts(&mut ctx));

    set_clipboard_conts(&mut ctx, String::from("saved-to-clipboard"));
    println!("{}", get_clipboard_conts(&mut ctx));
}
