use crate::{
    clipshare::{
        settings::{Settings, SettingsBuilder},
        device::{Dev, init_device},
    },
    connect::{
        discover::{DeviceMonitor, handle_new_client, SERVICE_TYPE},
        socket::{Address, setup_client, AddressBuilder},
    },
};

use std::{
    sync::{Arc, Mutex},
    net::Ipv4Addr,
};

pub struct Client {
    pub settings: Settings,
    pub device: Dev,
    pub address: Address,
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
