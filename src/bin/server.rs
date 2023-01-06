use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (sender, receiver) = unbounded();
    peer_map.lock().unwrap().insert(addr, sender);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients =
            peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

async fn setup_server(addr: String) -> TcpListener {

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(addr.clone()).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    listener
}

async fn spawn_server_conn(srv: TcpListener, state: PeerMap) {
    while let Ok((stream, addr)) = srv.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    //let srv = setup_server(addr).await;

    let try_socket = TcpListener::bind(addr.clone()).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    //let conn = spawn_server_conn(srv, state).await;
    let conn = spawn_server_conn(listener, state).await;

    // Let's spawn the handling of each connection in a separate task.
    //while let Ok((stream, addr)) = srv.accept().await {
        //tokio::spawn(handle_connection(state.clone(), stream, addr));
    //}

    Ok(())
}
