pub mod ws;

use clipshare::{Device, init_device, load_config};

fn test_clipboard(dev: &mut Device) {
    println!("{}", dev.get_clipboard_conts());
    dev.set_clipboard_conts(String::from("saved-to-clipboard"));
    println!("{}", dev.get_clipboard_conts());
}

fn main() {
    pretty_env_logger::init();

    let cfg = load_config();
    let mut dev = init_device();
    test_clipboard(&mut dev);
}
