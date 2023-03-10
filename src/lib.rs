pub mod app;
pub mod config;
pub mod consts;
pub mod discover;
pub mod ws;
#[cfg(target_os = "linux")]
#[cfg(feature = "gtk")]
pub mod gtk;
#[cfg(target_os = "windows")]
pub mod winui;

#[macro_use] extern crate log;

use clipboard::{ClipboardContext, ClipboardProvider};
use config::{get_clipboard_conts, set_clipboard_conts};

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
