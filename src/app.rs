// Main Functions
// - Device Monitor/Discoverer:
//      - Asynchronously watches for any new devices if they are emitting RegisterDevice requests
//          - If a device is found, then:
//              - The monitor responds to the device by sending DeviceRegistered,
//                  prompting the device to stop sending further requests to pair.
//              - The monitor passes control to the ClientMonitor by emitting RegisterClient requests
//                  with the necessary information available
//      - Runs forever
// - Device Register:
//      - Asynchronously sends DeviceRegister prompts every 5 seconds.
//      - Asynchronously listens for DeviceRegistered signals.
//          - If found, the await loop will terminate
//      - DeviceRegister service can be restarted by the main app
//      - Runs only as long as is needed to talk to the device monitor
// Client Monitor:
//      - Asynchronously watches for any new clients if they are emitting RegisterClient requests
//          - If a client is found, then:
//              - The client monitor will create a Client
//              - The client will be stored in a VecDeque of connected clients
//              - The client's connection will be added to a ConnectionPool of other clients
//      - Asynchronously watches the connection_pool of clients if they are emitting SendCopied requests
//          - If a SendCopied request was found, then:
//              - Copy the result to our clipboard 
//              - Emit UpdateDeviceHistory(Name, Conts) for the user interface
//      - Runs forever
// Client:
//      - Asynchronously listens to the main app channel
//      - If SendCopied is found then:
//          - Passthrough/Re-emit SendCopied(String) to the client_monitor channel
// App (No User Interface) / Daemon:
//      - Kills all previous running daemons.
//      - Creates the main app mpmc channel (with the receiver end dropped).
// User Interface:
//      - Asynchronously listens for:
//          - RegisterDeviceUntrusted(TODO)
//          - DeviceFound(TODO)
//          - UpdateDeviceHistory(Name, Conts, Time)
//              - This will update
//          - CloseApplication
//          - Reconnect
// App (User Interface):
//      - Kills all previous running daemons.
//      - Creates the main app mpmc channel (with the receiver end dropped).


// Create the discover daemon
// Create the clipshare-server daemon
// Create the clipshare-client daemon

// Start up the server, and the daemon (these will run on one thread)
// Start up the client on a separate thread

// If a new device is found by the daemon, issue a DeviceFound request to the server to add it
// The server maintains these hashmap of devices, and if DeviceFound, then we add the device to the hashmap

// New devices can be added using the discover-server functions which will issue an AddNewDevice request
// The main client will listen for the RegisterDevice request, and add it to the hashmap by issuing DeviceFound

// // The client will emit SendCopied requests to the other clients
// // The server will monitor clients for these SendCopied requests
// // If found then the server will
// // 1. Copy it to the clipboard

// The main app will emit SendCopied requests

// The server opens the connection to the device 

//use crate::{
    //config::{Settings, load_config},
    //ws::Dev,
//};

//pub struct App {
    //pub cfg: Settings,
    //pub device: Dev,
//}

//impl Default for App {
    //fn default() -> Self {
        //let cfg = load_config();
        //let dev = init_device();
        //let device = Arc::new(Mutex::new(dev));

        //Self { cfg, device }
    //}
//}

//impl App {
    //pub fn new() -> {
    //}
//}
