use clipshare::ws::{setup_server, poll_client_connections};
use clipshare::ws::PeerMap;

use std::{
    env,
    sync::Mutex,
    collections::HashMap,
};

#[tokio::main]
async fn main() {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = setup_server(addr.clone());
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    tokio::join!(poll_client_connections(server.await, state));
}
