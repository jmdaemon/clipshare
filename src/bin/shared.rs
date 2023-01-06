use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use futures::{future, join, channel, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// Server side
type Tx = UnboundedSender<Message>;
type Rx = UnboundedReceiver<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

//type PeerMapDuplex = Arc<Mutex<HashMap<SocketAddr, (Tx, Rx)>>>;
type PeerMapRx = Arc<Mutex<HashMap<String, Rx>>>;

async fn add_to_peer_map(peer_map: PeerMap, addr: SocketAddr) {
    // Insert the write part of this peer to the peer map.
    let (sender, receiver) = unbounded();
    peer_map.lock().unwrap().insert(addr, sender);
}

//async fn add_to_peer_map_duplex(peer_map: PeerMapDuplex, addr: SocketAddr) {
//async fn add_to_peer_map_duplex(peer_map: &PeerMapDuplex, addr: SocketAddr, sender: UnboundedSender<Message>, receiver: UnboundedReceiver<Message>) {
    //peer_map.lock().unwrap().insert(addr, unbounded());
    //peer_map.lock().unwrap().insert(addr, (sender, receiver));
//}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
//async fn handle_connection(peer_map: PeerMapDuplex, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    // Accept websocket connection
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
    // TODO: Save this websocket connection for later

    // Add sender, receiver parts to the duplex peer map
    let (sender, receiver) = unbounded();
    //add_to_peer_map_duplex(peer_map, addr);
    //add_to_peer_map_duplex(peer_map, addr, sender, receiver);
    //add_to_peer_map_duplex(&peer_map, addr, sender, receiver);
    //peer_map.lock().unwrap().insert(addr, (sender, receiver));
    peer_map.lock().unwrap().insert(addr, sender);

    // Use the websocket stream for things
    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        //let broadcast_recipients =
            //peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
        let broadcast_recipients =
            peers.iter().filter( |(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
            //peers.iter().filter( |(peer_addr, (sender, receiver))| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
            //peers.iter().filter( |(peer_addr, (sender, receiver))| peer_addr != &&addr).map(|(_, (ws_sink, ws_stream))| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });
    //let peer_map_mg = peer_map.lock().unwrap();
    //let receiver: &UnboundedReceiver<Message> = &peer_map.lock().unwrap().get(&addr).unwrap().1;
    //let receiver: &UnboundedReceiver<Message> = &peer_map_mg.get(&addr).unwrap().1;

    let receive_from_others = receiver.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

//async fn setup_server(addr: String) -> TcpListener {

    //// Create the event loop and TCP listener we'll accept connections on.
    //let try_socket = TcpListener::bind(&addr).await;
    //let listener = try_socket.expect("Failed to bind");
    //println!("Listening on: {}", addr);
    //listener
//}

async fn spawn_server_conn(srv: TcpListener, state: PeerMap) {
//async fn spawn_server_conn(srv: TcpListener, state: PeerMapDuplex) {
    while let Ok((stream, addr)) = srv.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }
}

//async fn setup_and_poll_server(addr: String) {
    //let state = PeerMap::new(Mutex::new(HashMap::new()));

    //// Setup server
    //let srv = setup_server(addr).await;

    //// Let's spawn the handling of each connection in a separate task.
    //let conn = spawn_server_conn(srv, state).await;

    //// Add connection 
//}

//async fn setup_and_connect_client

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    //let readers = PeerMapRx::new(Mutex::new(HashMap::new()));

    //let connect_addr = addr.clone();

    //setup_and_poll_server(addr);
    let index = 0;
    //let server = setup_server(index, addr.clone());
    //let client = setup_client(addr, connect_addr, readers);
    
    //let listener = server.await;
    let listener = setup_server(index, addr.clone()).await;
    //let ws_stream = client.await;

    //tokio::join!(
        //server,
        //client,
        //);

    spawn_server_conn(listener, state).await;
    //tokio::join!(
        //spawn_server_conn(listener, state),
        //client_send(ws_stream, readers)
        //);

    Ok(())
}

// Multiple server connections
// Multiple client connections


// Polls connection for potential clients
// This must be run and finished first
async fn setup_server(i: u64, addr: String) -> TcpListener {
    println!("Starting server {}.", i);

    // Awaits program
    // Polls connection for potential clients
    // Create the event loop and TCP listener we'll accept connections on.
    //let try_socket = TcpListener::bind(&addr).await;
    let try_socket = TcpListener::bind(&addr).await;
    //                                         ^ execution can be paused here
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    listener
}

// Attempts to connect to an available server
//async fn setup_client(addr: String, connect_addr: String) -> tokio_tungstenite::WebSocketStream<TcpStream> {
//async fn setup_client(addr: String, connect_addr: String) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>> {
async fn setup_client(
    addr: String,
    connect_addr: String,
    peer_map: PeerMapRx
    ) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>> {
    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    peer_map.lock().unwrap().insert(addr, stdin_rx);

    // If no servers are available, then act as the server
    //let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let request = connect_async(&url).await;
    //let ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>;

    //let request = request.unwrap();

    // FIXED:
    //let (ws_stream, _) = request;
    let (ws_stream, _) = request.unwrap();

    // FIXME: No matter if the server is started or not, we will also start our own server regardless
    //match request {
        //// If there is an error
        //Err(_) => {
            //// Start up the server
            //println!("No available servers. Starting server");
            //setup_server(0, addr).await;

            //// Try again
            //(ws_stream, _) = connect_async(&url).await.expect("Could not setup client");

            ////eprintln!("{}", err);
            ////std::process::exit(1);
        //}
        //// Else process the request as normal
        //Ok(request) => {
            ////let (ws_stream, _) = request;
            ////println!("No available servers. Starting server");
            //(ws_stream, _) = request;
        //}
    //}
    //let (ws_stream, _) = connect_async(url);
    println!("WebSocket handshake has been successfully completed");
    ws_stream

    // Now we want to store this websocket in the corresponding client data structure and return the websocket
    // We want to store 2 websockets, one for the client, and one for the server for every device (potential of 6 websockets total)
    // Then we want to treat all client websockets and all server websockets as just one concurrent pool each (for a total of 2 pools)
    // We'll use these pools to broadcast update requests to all the other client sockets, and to have the ability on every devie

    // We will manage 1 websocket per device
    // Every device will start up 1 server and 1 client respectively
    // When the websocket is connected, we will return this websocket and store this connection later in a pool for the devices



}

//async fn client_send(ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>, stdin_rx: UnboundedReceiver<Message>) {
async fn client_send(
    ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
    //addr: String,
    stdin_rx: UnboundedReceiver<Message>,
    //peer_map: PeerMapRx
    ) {
    //let pm_mg = peer_map.lock().unwrap();
    //let stdin_rx = pm_mg.get(&addr).to_owned().unwrap();
    // Split websocket into sender / receiver
    let (write, read) = ws_stream.split();

    // Default implementation
    // Send everything from stdin to everyone else connected on the websocket
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Read from the stdin channel and redirect it to the sender
async fn read_stdin(sender: channel::mpsc::UnboundedSender<Message>) {
    // Buffered io read
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        sender.unbounded_send(Message::binary(buf)).unwrap();
    }
}
