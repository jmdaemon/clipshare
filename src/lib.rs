// Standard Library
use std::collections::HashMap;

// Third Party Libraries
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};

// Data Structures
pub struct Device {
    pub name: String,
    pub history: Vec<String>,
    pub clipboard: ClipboardContext,
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
