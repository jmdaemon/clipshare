use clipshare::{init_device, load_config};

#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    let cfg = load_config();
    let mut dev = init_device();

    println!("{}", dev.get_clipboard_conts());
    dev.set_clipboard_conts(String::from("saved-to-clipboard"));
    println!("{}", dev.get_clipboard_conts());
}
