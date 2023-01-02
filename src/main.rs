// Standard Library
use std::collections::HashMap;

// Third Party Libraries
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use serde::{Serialize, Deserialize};

// Data Structures
pub struct Device {
    name: String,
    history: Vec<String>
}

type Shortcuts = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    max_history: u64,
    //shortcuts: Shortcuts,
}


// Clipboard
pub fn get_clipboard_conts() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}


//impl Config {
    //pub fn default(&self) -> Config {
        //let shortcuts = HashMap::from([
                //(String::from("Enable/Disable Device"), String::from("Ctrl + {}")),
        //]);
        //self.new(10_000, shortcuts)
    //}
    //pub fn new(&self, max_history: u64, shortcuts: Shortcuts) -> Config {
        ////Config { max_history, shortcuts }
        //Config { max_history }
    //}
//}

fn main() {

    // Init
    //let cfg = Config::default;
    let cfg = Config { max_history: 10_000 };

    let cfg_json = serde_json::to_string(&cfg).unwrap();
    println!("Config:\n{}", cfg_json);


    println!("{}", get_clipboard_conts());
}
