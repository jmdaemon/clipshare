// Common Imports
use crate::clipshare::device::{Dev, Device};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    join,
};

// Client Imports
use tokio_tungstenite::connect_async;

// Server Imports
use std::{
    fmt,
    collections::HashMap,
    net::{SocketAddr, Ipv4Addr},
    sync::{Arc, Mutex},
    str::FromStr,
    num::ParseIntError,
};

use futures::{
    future,
    pin_mut,
    StreamExt,
    stream::{TryStreamExt},
};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    sync::broadcast::{Sender, Receiver},
};

use tokio_tungstenite::tungstenite::protocol::Message;

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



// Types
pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// Async
/// Listen for client connections
pub async fn setup_server(addr: String) -> TcpListener {
    let try_socket = TcpListener::bind(addr.clone()).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    listener
}

/// Establish connection to client, and register our event listener callback function
fn cb_server_message_received(dev: &mut Dev, msg: Message, peer_map: &PeerMap, addr: SocketAddr) -> future::Ready<Result<(), tungstenite::Error>> {
    println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
    let peers = peer_map.lock().unwrap();

    println!("Copying to clipboard");
    let conts = msg.to_string().trim().to_string();
    dev.lock().unwrap().set_clipboard_conts(conts);

    // We want to broadcast the message to everyone except ourselves.
    let broadcast_recipients =
        peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

    for recp in broadcast_recipients {
        recp.unbounded_send(msg.clone()).unwrap();
    }

    future::ok(())
}

pub async fn handle_connection(mut dev: Dev, peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    // Accept and create the websocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (sender, receiver) = unbounded();
    peer_map.lock().unwrap().insert(addr, sender);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        cb_server_message_received(&mut dev, msg, &peer_map, addr)
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

// Accepts new client connections
pub async fn poll_client_connections(dev: Dev, srv: TcpListener, state: PeerMap) {
    while let Ok((stream, addr)) = srv.accept().await {
        tokio::spawn(handle_connection(dev.clone(), state.clone(), stream, addr));
    }
}

pub async fn clipboard_changed(dev: &Dev) -> String {
    let mut lock_dev = dev.lock().unwrap();
    let conts = lock_dev.get_clipboard_conts();
    loop {
        let now = lock_dev.get_clipboard_conts();
        if conts != now {
            break now;
        }
    }
}

pub async fn send_on_clipboard_change(tx: UnboundedSender<Message>, dev: Dev) {
    let clone_dev = dev.clone();
    loop {
        let conts = clipboard_changed(&clone_dev).await;
        println!("Clipboard Contents: {}", &conts);

        // Send contents to connected device
        tx.unbounded_send(Message::text(conts));
    };
}

pub async fn setup_client(dev: Dev, connect_addr: String) {
    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = unbounded();
    tokio::spawn(send_on_clipboard_change(stdin_tx, dev));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, _read) = ws_stream.split();
    let clipboard_to_ws = stdin_rx.map(Ok).forward(write);
    join!(clipboard_to_ws);
}
