use std::collections::HashMap;

use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};

#[macro_use] extern crate log;

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

    pub fn get_clipboard_conts(&mut self) -> String {
        get_clipboard_conts(&mut self.clipboard)
    }

    pub fn set_clipboard_conts(&mut self, conts: String) {
        set_clipboard_conts(&mut self.clipboard, conts)
    }
}

type Shortcuts = HashMap<String, String>;

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
