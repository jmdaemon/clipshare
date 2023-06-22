#[macro_use] extern crate log;
use clipshare::{
    ui::gtk::widgets::app::{App, AppInit},
    consts::APP_ID
};
use relm4::{RelmApp, set_global_css_from_file};

fn main() {
    pretty_env_logger::init();
    set_global_css_from_file("src/gtk/clipshare.css");
    let app = RelmApp::new(APP_ID);
    let init = AppInit {};
    app.run::<App>(init);
}
