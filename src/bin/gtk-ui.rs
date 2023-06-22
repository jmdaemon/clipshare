#[macro_use] extern crate log;
use clipshare::{
    ui::gtk::widgets::app::{App, AppInit},
    consts::APP_ID
};
use relm4::RelmApp;

fn main() {
    pretty_env_logger::init();

    let app = RelmApp::new(APP_ID);
    let init = AppInit {};
    app.run::<App>(init);
}
