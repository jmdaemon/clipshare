// Imports
use clipshare::{Device, Config, get_clipboard_conts, set_clipboard_conts};

// Third Party Libraries
use clipboard::{ClipboardContext, ClipboardProvider};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

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
