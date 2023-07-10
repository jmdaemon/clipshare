use clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardManager {
    pub clipboard: ClipboardContext,
}

impl Default for ClipboardManager {
    fn default() -> Self {
        let clipboard: ClipboardContext = ClipboardProvider::new()
            .expect("Could not create clipboard context");
        Self { clipboard }
    }
}

impl ClipboardManager {
    pub fn new() ->  Self {
        Default::default()
    }

    pub fn get_clipboard_conts(&mut self) -> String {
        self.clipboard.get_contents().unwrap()
    }

    pub fn set_clipboard_conts(&mut self, conts: String) {
        self.clipboard.set_contents(conts).expect("Could not set contents of clipboard");
    }
}
