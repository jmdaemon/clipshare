use clipshare::ws::{setup_server,setup_client, poll_client_connections};
use clipshare::ws::PeerMap;

use std::{
    env,
    sync::Mutex,
    collections::HashMap,
};

use futures::future::join_all;

#[tokio::main]
async fn main() {
    // Setup the broadcast channel
    //let cb_chan = ClipboardChannel::new();
    //let (s, mut r) = (cb_chan.s, cb_chan.r);

    // Setup our subscribers
    //let mut dev_r2 = s.subscribe();
    //let mut dev_r3 = s.subscribe();

    //let last_copied = "string-to-be-copied";
    // TODO: Need to make broadcast channel to signal and notify clipboard changes to server thread
    // TODO: Need to send multiple client connections to other devices
    // TODO: Need to make function to add client/remove device clients.

    // We want to set up our devices 

    // Set up our server to listen for incoming connections
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = setup_server(addr.clone());

    // TODO: Load the cached device configs and 
    // Have other devices connect to our device as a server
    let mut clients = vec![];
    for i in 0..3 {
        println!("Setup client #{}", i+1);
        let url = format!("ws://{}", addr.clone());
        clients.push(setup_client(url));
    }

    // Send client connection requests to other devices

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    tokio::join!(
        join_all(clients),
        poll_client_connections(server.await, state),
        );
}
