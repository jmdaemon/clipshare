use std::fmt;

// Watch/Subscribe (to) our device clipboards' contents
// If the contents have changed
// Notify/Publish the changes to UnifiedClipboard

// Watch the last_copied string
// If the contents change
// Then for each device clipboard,
//      send out the last_copied String

// If we get a new post/receive some new data 
// Interpret message/response
// Set current device's clipboard to the new data

// We need to use a websocket for bidirectional communication
// We need to setup two services, one to act as the server, and another for the client.
pub struct Address {
    pub ip: String,
    pub port: u32,
}

// Traits
// - connect

// Threading:
// We'll need at least 3 threads to do io
// 1. Thread to run the main GUI rendering
// 2. Another thread to run the client/server websocket
pub struct Client {
    pub addr: Address,
    // Channel
}

impl Client {
    pub fn connect(&self, to: &Server) {
        // Set up websocket connection to available server devices
        // We will send the response to accept the connection with the server on our client
        //if let Err(error) = listen(to.addr.fmt(), |out| {
        // The handler needs to take ownership of out, so we use move

        //move |msg| {
            //// Handle messages received on this connection
            //println!("Server got message '{}'. ", msg);

            //// Use the out channel to send messages back
            //out.send(msg)
        //}
    //}) {
        //// Inform the user of failure
        //println!("Failed to create WebSocket due to {:?}", error);
    //}
    //}

    //pub fn receive(&self, _src: &Device) {
        //// Receive the clipboard contents from the src device
    //}
    }
}

pub struct Server {
    pub addr: Address,
    // Separate Channel
}

impl Server {
    pub fn connect(&self) {
        // Set up websocket connection to available client devices
        // We will send the request to connect to available devices
    }

    pub fn send(&self, _dest: &Device) {
        // Send clipboard contents from our device to the dest device

    }
}

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
