use crate::lines_from_file;
use clipboard::{ClipboardContext, ClipboardProvider};

use std::path::Path;

pub struct Device {
    pub name: String,
    pub history: Vec<String>,
    pub clipboard: ClipboardContext,
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

// Clipboard
pub fn get_clipboard_conts(ctx: &mut ClipboardContext) -> String {
    ctx.get_contents().unwrap()
}

pub fn set_clipboard_conts(ctx: &mut ClipboardContext, conts: String) {
    ctx.set_contents(conts).expect("Could not set contents of clipboard");
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

    //pub fn history_from_file(&mut self, path: &Path) -> Device {
    //}
}

#[derive(Default)]
pub struct DeviceBuilder {
    pub name: Option<String>,
    pub history: Option<Vec<String>>,
}

impl DeviceBuilder {
    pub fn new() -> Self { Default::default() }
    pub fn build(&self) -> Device {
        let clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        Device { name: self.name.to_owned().unwrap(), history: self.history.to_owned().unwrap(), clipboard }
    }

    pub fn from_hostname(mut self) -> DeviceBuilder {
        self.name = Some(hostname::get().expect("Could not get os hostname").into_string().unwrap());
        self
    }

    pub fn read_history_from_file<P>(mut self, path: P) -> DeviceBuilder
    where
        P: AsRef<Path>,
    {
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
    let dev_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let hostname = hostname::get();
    let dev = Device::new(String::from("Device 1"), vec![String::from("asdf")], dev_ctx);
    info!("Device Name: {}", dev.name);
    info!("Device History:");
    dev.history.iter().for_each( |line| {
        info!("{}", line);
    });
    dev
}

