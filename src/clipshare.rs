// Client Imports
use crate::{
    config::{Settings, SettingsBuilder},
    device::init_device,
    discover::{DeviceMonitor, handle_new_client, SERVICE_TYPE},
    ws::{Dev, Address, setup_client, AddressBuilder},
};

use std::sync::{Arc, Mutex};

// Server Imports
use crate::{
    discover::Device,
    ws::{PeerMap, poll_client_connections, setup_server}
};

use std::{
    net::Ipv4Addr,
    thread::sleep,
    time::Duration,
    collections::HashMap,
};

use local_ip_address::local_ip;

pub struct Client {
    pub settings: Settings,
    pub device: Dev,
    pub address: Address,
}

pub struct Server {
    pub settings: Settings,
    pub device: Dev,
    pub address: Arc<Address>,
}

// Client
impl Default for Client {
    fn default() -> Self {
        let settings = SettingsBuilder::new().build();
        let device = Arc::new(Mutex::new(init_device()));
        let address = AddressBuilder::new().build();
        Self { settings, device, address }
    }
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }

    /// Scans for available devices in the LAN and obtains an ip address to the first
    /// available device. In the future, this should use an address pool, and a separate channel
    /// to manage addition/removal of devices
    pub async fn monitor_devices(&self) -> Ipv4Addr {
        let device_monitor = DeviceMonitor::new(SERVICE_TYPE);
        let mut device_addr = Ipv4Addr::UNSPECIFIED;
        while let Ok(event) = device_monitor.receiver.recv_async().await {
            if let Some(service_info) = handle_new_client(&event) { 
                let addresses = service_info.get_addresses();
                addresses.iter().for_each(|address| {
                    device_addr = address.to_owned();
                });
                if device_addr != Ipv4Addr::UNSPECIFIED {
                    break;
                }
            }
        }
        device_addr
    }

    /// Establishes the websocket connection to the device at address
    pub async fn connect_to(&self, address: Address) {
        println!("Connecting to device {}", address);

        // Create client to the connected device
        //let url = format!("ws://{}:{}", addr, port);
        let url = format!("ws://{}", address);
        //let client = setup_client(url);
        let client = setup_client(self.device.to_owned(), url);
        tokio::join!(client);
    }
}

// Server
async fn start_server(dev: Dev, address: Arc<Address>) {
    //let addr = address.to_string();
    //let server = setup_server(addr.clone());
    let server = setup_server(address.to_string());
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    tokio::join!(poll_client_connections(dev, server.await, state));
}

// Attempt to register device
async fn register_device(address: Arc<Address>) {
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    let addr = &address.ip;
    let host_ipv4 = addr;
    let host_name = format!("{}.local.", host_ipv4);
    let port = address.port as u16; // FIXME: Make sure to change port data type
    let properties = [("property_1", "test"), ("property_2", "1234")];
    
    let device = Device::new(service_type, instance_name, host_ipv4, &host_name, port, &properties[..]);
    if let Some(service) = &device.service_info {
        // TODO: Run forever for now, we'll kill this later somehow,
        // when we drop the connection.
        loop {
            // Register with the daemon, which publishes the service.
            device.mdns.register(service.to_owned()).expect("Failed to register our service");
            sleep(Duration::from_secs(15));

            //let results = device.mdns.get_metrics().unwrap().into_recv_async().await.unwrap();
            //results.iter().for_each(|(k, v)| {
                //println!("Metrics Key: {}\nMetrics Value: {}", k, v);
            //});
        }
    }
    
}

impl Default for Server {
    fn default() -> Self {
        let settings = SettingsBuilder::new().build();
        let device = Arc::new(Mutex::new(init_device()));
        let address = Arc::new(
            AddressBuilder::new()
            .ip(local_ip().unwrap().to_string())
            .build());
        Self { settings, device, address }
    }
}


impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn start(&self) {
        let addr_register_device = self.address.clone();
        let addr_websocket_server = self.address.clone();

        // Create async task
        let thread_register_device = tokio::spawn(async move {
            register_device(addr_register_device).await
        });

        // In another thread, establish the server
        let device = self.device.clone();
        let thread_start_server = tokio::spawn(async move {
            start_server(device, addr_websocket_server).await
        });

        let joined = tokio::join!(
            thread_register_device,
            thread_start_server,
        );
        joined.0.expect("Could not register device");
        joined.1.expect("Could not start server");
    }
}
