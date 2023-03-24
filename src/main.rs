use clipshare::{
    clipshare::{
        device::{Device, init_device},
        settings::Settings,
    },
};
#[macro_use] extern crate log;

fn test_clipboard(dev: &mut Device) {
    println!("{}", dev.get_clipboard_conts());
    dev.set_clipboard_conts(String::from("saved-to-clipboard"));
    println!("{}", dev.get_clipboard_conts());
}

fn main() {
    pretty_env_logger::init();
    info!("Copying to clipboard");

    let _settings = Settings::default().load_config();
    let mut dev = init_device();
    test_clipboard(&mut dev);
}
