use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread, vec,
};

use futures::{channel::mpsc::{unbounded, UnboundedSender}, stream::{SplitStream, ForEach}, TryFutureExt};
use futures::{channel, future, pin_mut, stream::TryStreamExt, StreamExt};

//use tokio::sync::broadcast;
//use tokio::sync::broadcast::error::{SendError, RecvError};
//use tokio::sync::broadcast::{Sender, Receiver};
//use tokio::time::Duration;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::broadcast,
    sync::broadcast::{Sender, Receiver, error::{SendError, RecvError}},
    time::Duration,
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

//use tokio::net::{TcpListener, TcpStream};
//use tokio::stream::SplitStream;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;


// Execution model
// We send clipboard events over this channel
// we also listen for clipboard events over this channel

#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    ReceiveCopied(String),
}

pub struct ClipboardChannel {
    s: Sender<ClipboardEvent>,
    r: Receiver<ClipboardEvent>,
}

impl ClipboardChannel {
    pub fn new() -> ClipboardChannel {
        let (s, r): (Sender<ClipboardEvent>, Receiver<ClipboardEvent>) = broadcast::channel(32);
        ClipboardChannel { s, r }
    }
}

/// Callback to notify listeners for a clipboard update
pub fn cb_send_update(r: Sender<ClipboardEvent>, last_copied: &str) -> Result<usize, SendError<ClipboardEvent>> {
    let event = ClipboardEvent::ReceiveCopied(String::from(last_copied));
    r.send(event)
}

/// Callback to parse notification from senders for a clipboard update
pub async fn cb_receive_update(r: &mut Receiver<ClipboardEvent>) {
    let res = r.recv().await.unwrap();
    match res {
        ClipboardEvent::ReceiveCopied(last_copied) => {
            println!("Received {}", last_copied);
        }
    }
}

// Setup 
async fn setup_server(addr: String) -> TcpListener {

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(addr.clone()).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    listener
}

/// Establish connection to client, and register our event listener callback function
fn cb_server_message_received(msg: Message, peer_map: &PeerMap, addr: SocketAddr) -> future::Ready<Result<(), tungstenite::Error>> {
    println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
    let peers = peer_map.lock().unwrap();

    // We want to broadcast the message to everyone except ourselves.
    let broadcast_recipients =
        peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

    for recp in broadcast_recipients {
        recp.unbounded_send(msg.clone()).unwrap();
    }

    future::ok(())
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
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
        cb_server_message_received(msg, &peer_map, addr)
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}


// Accepts new client connections
async fn poll_client_connections(srv: TcpListener, state: PeerMap) {
    while let Ok((stream, addr)) = srv.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }
}

//async fn cb_client_message_received(read: SplitStream<WebSocketStream<TcpStream>>, message: Message) -> Result<Message, Error> {
//async fn cb_client_message_received(read: SplitStream<WebSocketStream<TcpStream>>, message: Message) {
//async fn cb_client_message_received(read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>) {

//async fn cb_client_message_received(read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>) -> ForEach<SplitStream<WebSocketStream<TcpStream>>>{
    //let ws_to_stdout = {
        //read.for_each(|message| async {
            //let data = message.unwrap().into_data();
            //tokio::io::stdout().write_all(&data).await.unwrap();
        //})
    //};
    //ws_to_stdout
//}

async fn setup_client(connect_addr: String) {
    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    //let ws_to_stdout = cb_client_message_received(read);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };
    //let ws_to_stdout = {
        //cb_client_message_received(message)
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

#[tokio::main]
async fn main() {
    // Setup the broadcast channel
    //let cb_chan = ClipboardChannel::new();
    //let (s, mut r) = (cb_chan.s, cb_chan.r);

    // Setup our subscribers
    //let mut dev_r2 = s.subscribe();
    //let mut dev_r3 = s.subscribe();

    //let last_copied = "string-to-be-copied";


    // We want to set up our devices 
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //let connect_addr = addr.clone();
    let server = setup_server(addr.clone());

    //let mut clients = vec![];
    //for _ in 0..3 {
        //clients.push(setup_client(&addr));
    //}
    //for _ in 0..3 {
        //setup_client(&addr).await;
    //}
    let client_handles = tokio::spawn(
        async move {
            for i in 0..3 {
                println!("Setup client #{}", i);
                let url = format!("ws://{}", addr.clone());
                setup_client(url).await;
            }
        }).unwrap_or_else(|_| {});
    
    //server.await;
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    tokio::join!(
        poll_client_connections(server.await, state),
        client_handles,
        //&clients[0],

        );


    //let client = 

    // Our subscribers will poll for data, the data will be moved in here
    //let handle_dev1 = tokio::spawn(async move {
            //cb_receive_update(&mut r).await;
        //}).unwrap_or_else(|_| {});
    //let handle_dev2 = tokio::spawn(async move {
            //cb_receive_update(&mut dev_r2).await;
        //}).unwrap_or_else(|_| {});
    //let handle_dev3 = tokio::spawn(async move {
            //cb_receive_update(&mut dev_r3).await;
        //}).unwrap_or_else(|_| {});
    //let handle_wait = async move {
            //tokio::time::sleep(Duration::from_secs(5)).await;
        //};

    //let handle_sender = async move {
            //match cb_send_update(s, last_copied) {
                //Ok(_) => {},
                //Err(e) => {
                    //eprintln!("Could not send data.");
                    //eprintln!("Error: {}", e);
                //}
            //}
    //};


    //tokio::join!(
        //// Spawn three separate subscribers
        ////handle_wait,    // Wait a bit
        //handle_dev1,
        //handle_dev2,
        //handle_dev3,
    //);
    //handle_wait.await;
    ////handle_sender,  // Send one update
    //handle_sender.await; // Send one update
}
