/*
use clipshare::ws::setup_client;
use std::env;

#[tokio::main]
async fn main() {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let url = format!("ws://{}", addr.clone());
    let client = setup_client(url);
    tokio::join!(client);
}
*/
