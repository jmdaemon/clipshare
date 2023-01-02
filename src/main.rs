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

    let cfg = Config::default();
    let cfg_json = serde_json::to_string(&cfg).unwrap();
    info!("Config:\n{}", cfg_json);


    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{}", get_clipboard_conts(&mut ctx));

    set_clipboard_conts(&mut ctx, String::from("saved-to-clipboard"));
    println!("{}", get_clipboard_conts(&mut ctx));
}
