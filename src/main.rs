extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn get_clipboard_conts() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

fn main() {
    println!("{}", get_clipboard_conts());
}
