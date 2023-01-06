use std::fmt;

/*
 * 1. Initialize event bus
 * 2. Create async task: notify_changed() -> { Send ClipboardEvent::LastCopiedChanged() }
 * 3. Create async task: on_changed() -> { Listen for ClipboardEvent::LastCopiedChanged(), and send App::UpdateClientRequest(String) }
 * 4. In server, listen for App events, and notify to all clients
 * 5. In client, listen for App event, and sync local changes to current_device clipboard
 */

pub struct Address {
    pub ip: String,
    pub port: u32,
}

pub struct Client {
    pub addr: Address,
}

pub struct Server {
    pub addr: Address,
}

// Implementations

// In the future, more features may be desired such as the ability to send over more than just copied
// clipboard text, but also files. May require another interface.

// TODO: Implement pairing request functionality
// TODO: Implement server event bus notifying 
// TODO: Implement server pair_request function 
// Sends the signal to the server, that when detected & matched for, adds the client to the current vector of futures)
// Use two async tasks for this, one to add, one to listen for client additions
// enum: AddDevice, RemoveDevice

// TODO: Implement local device ip/scan for quickly adding detected devices.
// TODO: Implement bluetooth scanning

// TODO: Cache device configuration details to disk. Use serde

impl Address {
    pub fn fmt(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt())
    }
}

impl Client { }
impl Server { }
