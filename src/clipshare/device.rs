use crate::{lines_from_file, clipboard::ClipboardManager};

use std::{
    sync::{Arc, Mutex},
    path::Path,
};

pub type Dev = Arc<Mutex<Device>>;

pub struct Device {
    pub name: String,
    pub history: Vec<String>,
    pub clipboard_manager: ClipboardManager,
}

// Implementations
//impl Default for Device {
    //fn default() -> Self {
        //let name = hostname::get().expect("Could not get os hostname").into_string().unwrap();
        //let history = vec![];
        //let clipboard = ClipboardProvider::new().unwrap();
        //Self { name, history, clipboard }
    //}
//}

impl Device {
    pub fn new(name: String, history: Vec<String>, clipboard_manager: ClipboardManager) -> Device {
        Device { name, history, clipboard_manager }
    }
}

#[derive(Default)]
pub struct DeviceBuilder {
    pub name: Option<String>,
    pub history: Option<Vec<String>>,
}

impl DeviceBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> Device {
        let clipboard_manager = ClipboardManager::new();
        Device { name: self.name.to_owned().unwrap(), history: self.history.to_owned().unwrap(), clipboard_manager }
    }

    pub fn from_hostname(mut self) -> DeviceBuilder {
        self.name = Some(hostname::get().expect("Could not get os hostname").into_string().unwrap());
        self
    }

    pub fn read_history_from_file(mut self, path: impl AsRef<Path>) -> DeviceBuilder {
        let conts = lines_from_file(path).expect("Could not read lines from file.");
        let history: Vec<String> = conts.map(|line| line.unwrap()).collect();
        self.history = Some(history);
        self
    }
}

//pub fn init_device() -> Device {
    //DeviceBuilder::new()
        //.from_hostname()
        //.read_history_from_file()
        //.build()
//}


// Helper test method to quickly initialize a dummy device
pub fn init_device() -> Device {
    let hostname = hostname::get();
    let clipboard_manager = ClipboardManager::new();
    let dev = Device::new(String::from("Device 1"), vec![String::from("asdf")], clipboard_manager);
    info!("Device Name: {}", dev.name);
    info!("Device History:");
    dev.history.iter().for_each( |line| {
        info!("{}", line);
    });
    dev
}

