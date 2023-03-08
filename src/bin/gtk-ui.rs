use clipshare::{
    gtk::widgets::app::{App, AppInit},
    consts::APP_ID
};
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new(APP_ID);
    let init = AppInit {};
    app.run::<App>(init);
}
