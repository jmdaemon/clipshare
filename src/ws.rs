use std::{
    fmt,
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread, vec,
};

use futures::{
    channel, future, pin_mut, StreamExt,
    channel::mpsc::{unbounded, UnboundedSender},
    stream::{TryStreamExt}, TryFutureExt,
    future::join_all,
};
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::broadcast,
    sync::broadcast::{Sender, Receiver, error::{SendError, RecvError}},
    time::Duration,
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use local_ip_address::local_ip;

use crate::Device;

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

// Types
pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
pub type Dev = Arc<Mutex<Device>>;

#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    ReceiveCopied(String),
}

pub struct ClipboardChannel {
    s: Sender<ClipboardEvent>,
    r: Receiver<ClipboardEvent>,
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

    pub fn from_str(s: String) -> Address {
        let addr_vec: Vec<&str> = s.split(":").collect();
        //let ip = addr_vec[0];
        //let port: u32 = addr_vec[1].parse().unwrap();
        let (ip, port) = (addr_vec[0], addr_vec[1].parse().unwrap());
        //let port: u32 = addr_vec[1].parse().unwrap();

        Address { ip: ip.to_owned(), port }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt())
    }
}

impl Client { }
impl Server {
    pub fn get_ip_addr() -> Address {
        let my_local_ip = local_ip().unwrap();
        Address::from_str(my_local_ip.to_string())
    }
}

// Async
impl ClipboardChannel {
    pub fn new() -> ClipboardChannel {
        let (s, r): (Sender<ClipboardEvent>, Receiver<ClipboardEvent>) = broadcast::channel(32);
        ClipboardChannel { s, r }
    }
}

///// Callback to notify listeners for a clipboard update
//pub fn cb_send_update(r: Sender<ClipboardEvent>, last_copied: &str) -> Result<usize, SendError<ClipboardEvent>> {
//    let event = ClipboardEvent::ReceiveCopied(String::from(last_copied));
//    r.send(event)
//}
//
///// Callback to parse notification from senders for a clipboard update
//pub async fn cb_receive_update(r: &mut Receiver<ClipboardEvent>) {
//    let res = r.recv().await.unwrap();
//    match res {
//        ClipboardEvent::ReceiveCopied(last_copied) => {
//            print!("Received {}", last_copied);
//            dev.set_clipboard_conts(last_copied);
//        }
//    }
//}

/// Listen for client connections
pub async fn setup_server(addr: String) -> TcpListener {
    let try_socket = TcpListener::bind(addr.clone()).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    listener
}

/// Establish connection to client, and register our event listener callback function
//fn cb_server_message_received(msg: Message, peer_map: &PeerMap, addr: SocketAddr) -> future::Ready<Result<(), tungstenite::Error>> {
fn cb_server_message_received(dev: &mut Dev, msg: Message, peer_map: &PeerMap, addr: SocketAddr) -> future::Ready<Result<(), tungstenite::Error>> {
    print!("Received a message from {}: {}", addr, msg.to_text().unwrap());
    let peers = peer_map.lock().unwrap();
    println!("Copying to clipboard");
    //dev.lock().unwrap().set_clipboard_conts(msg.to_string());
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

    //incoming.try_for_each()
    let broadcast_incoming = incoming.try_for_each(|msg| {
        //cb_server_message_received(msg, &peer_map, addr)
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

pub async fn setup_client(mut dev: Dev, connect_addr: String) {
    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();
    
    //let data = dev.get_clipboard_conts();
    //let data = dev.lock().unwrap().get_clipboard_conts();
    //tokio::io::stdout().write_all(&data.as_bytes()).await.unwrap();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            //let data = message.unwrap().into_data();
            //tokio::io::stdout().write_all(&data).await.unwrap();
            let data = dev.lock().unwrap().get_clipboard_conts();
            tokio::io::stdout().write_all(&data.as_bytes()).await.unwrap();
        })
    };

    //let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    //let ws_to_stdout = {
        //read.for_each(|message| async {
            ////let data = message.unwrap().into_data();
            //tokio::io::stdout().write_all(&data).await.unwrap();
        //})
    //};

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
