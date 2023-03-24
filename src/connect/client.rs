use crate::{
    clipshare::{
        settings::{Settings, SettingsBuilder},
        device::{Dev, init_device},
    },
    connect::{
        address::{Address, AddressBuilder},
        discover::{ServiceFinder, SERVICE_TYPE},
        socket::setup_client,
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
    pub async fn get_device_addresses(&self) -> Vec<Ipv4Addr> {

        // Find devices with the service
        let service_finder = ServiceFinder::new(SERVICE_TYPE);
        let timeout = std::time::Duration::from_secs(30);
        let found = service_finder.find_devices(timeout).await;

        // Yield all the addresses
        //let addresses: Vec<Ipv4Addr> = found.into_iter().map(|info| {
        //info.get_addresses().into_iter()
            //.map(|addr| addr.to_owned())
            //.collect::<Vec<Ipv4Addr>>()
        //}).flatten().collect();
        //addresses

        found.into_iter().flat_map(|info| {
            info.get_addresses().iter()
                .map(|addr| addr.to_owned())
                .collect::<Vec<Ipv4Addr>>()
        }).collect()


        //}).fold(vec![], |acc, addresses| acc.push(addresses));
        //addresses
        //let mut first_address = Ipv4Addr::UNSPECIFIED;

        //while let Ok(event) = service_finder.receiver.recv_async().await {
            //if let Some(service_info) = handle_new_client(&event) { 
                //let addresses = service_info.get_addresses();
                //addresses.iter().for_each(|address| {
                    //device_addr = address.to_owned();
                //});
                //if device_addr != Ipv4Addr::UNSPECIFIED {
                    //break;
                //}
            //}
        //}
        //device_addr
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
