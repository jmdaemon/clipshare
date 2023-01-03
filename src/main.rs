use clipshare::{init_device, Config};

#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    let cfg = Config::default();
    let cfg_json = serde_json::to_string(&cfg).unwrap();
    info!("Config:\n{}", cfg_json);

    let mut dev = init_device();

    println!("{}", dev.get_clipboard_conts());

    dev.set_clipboard_conts(String::from("saved-to-clipboard"));
    println!("{}", dev.get_clipboard_conts());
}
