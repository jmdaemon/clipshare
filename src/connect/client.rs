use futures::future::join;
use mdns_sd::ServiceInfo;

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
    net::Ipv4Addr, time::Duration,
};

pub struct Client {
    pub settings: Settings,
    pub device: Dev,
    pub address: Address,
}

pub enum ClientMessage {
    DeviceFound,
}

pub type Clients = Arc<Mutex<Vec<Client>>>;

pub struct ClientPool {
    pub clients: Clients,
}

// Client
impl Default for Client {
    fn default() -> Self {
        let settings = SettingsBuilder::new().build();
        let device = Arc::new(Mutex::new(init_device()));
        let address = AddressBuilder::default().build().expect("Could not create AddressBuilder");
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
    //pub async fn get_device_addresses(&self) -> Vec<Ipv4Addr> {
    pub async fn get_device_address(&self) -> Ipv4Addr {

        // Find devices with the service
        let service_finder = ServiceFinder::new(SERVICE_TYPE);
        let timeout = std::time::Duration::from_secs(10);

        // We must populate and parse on the devices at 
        //let device_pool = Arc::new(Mutex::new(vec![]));

        //let found = service_finder.find_devices(timeout).await;

        //let found = service_finder.find_devices(timeout, device_pool).await;

        let serv_info = service_finder.find_device(timeout).await;

        let parse_address = |info: ServiceInfo| {
            info.get_addresses().iter()
                .map(|addr| addr.to_owned())
                .collect::<Vec<Ipv4Addr>>()
        };
        let ip = parse_address(serv_info.unwrap());
        ip.first().unwrap().to_owned()
        /*
        let parse_address = |info: ServiceInfo| {
            info.get_addresses().iter()
                .map(|addr| addr.to_owned())
                .collect::<Vec<Ipv4Addr>>()
        };

        let parse_addresses = |found: Vec<ServiceInfo> | {
            found.into_iter().flat_map(|info| {
                parse_address(info)
                //info.get_addresses().iter()
                    //.map(|addr| addr.to_owned())
                    //.collect::<Vec<Ipv4Addr>>()
            }).collect()
        };

        //let handle_find_devices = service_finder.find_devices(timeout, device_pool);
        let time = Duration::from_secs(30);

        let device_pool = Arc::new(Mutex::new(vec![]));
        let handle_parse_addresses = async || {
            if let Some(info) = service_finder.find_device(time).await {
                let addresses = parse_address(info);
                let addr = addresses.first().unwrap();
                //let addr = addr.to_string();

                // If the address is a new address
                let dev_pool_handle = device_pool.clone().lock().unwrap();
                if !dev_pool_handle.contains(addr) {
                    // Add it to the device_pool
                    dev_pool_handle.push(addr);

                    // Run the async the new address
                }
            }
        };

        join!(future1, future2)

    */
        //parse_addresses(found);

        // Parse and return just their addresses
        
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

impl Default for ClientPool {
    fn default() -> Self {
        Self { clients: Arc::new(Mutex::new(vec![])) }
    }
}

impl ClientPool {
    pub fn new() -> Self {
        Default::default()
    }

    /// Scans for available devices in the LAN and obtains an ip address to the first
    /// available device. In the future, this should use an address pool, and a separate channel
    /// to manage addition/removal of devices
    //pub async fn get_device_addresses(&self) -> Ipv4Addr {
    //pub async fn get_device_addresses(&self) {
    pub async fn populate(&self) -> Self {

        // Find devices with the service
        let service_finder = ServiceFinder::new(SERVICE_TYPE);
        let timeout = std::time::Duration::from_secs(10);

        let parse_address = |info: ServiceInfo| {
            info.get_addresses().iter()
                .map(|addr| addr.to_owned())
                .collect::<Vec<Ipv4Addr>>()
        };

        //let parse_addresses = |found: Vec<ServiceInfo> | {
            //found.into_iter().flat_map(parse_address).collect::<Vec<Ipv4Addr>>()
        //};

        //parse_addresses();

        let serv_info = service_finder.find_device(timeout).await;
        let addresses = parse_address(serv_info.unwrap());
        
        let clients: Vec<Client> = addresses.iter().map(|address| {
            let mut client = Client::new();
            let address = AddressBuilder::default()
                .ip(address.to_string())
                .build()
                .unwrap();
            //let address = AddressBuilder::ip(address.to_string()).build();
            //let address = Address::from(address.to_string().as_str());
            client.address = address;
            client
        }).collect();

        Self { clients: Arc::new(Mutex::new(clients)) }
        //let ip = parse_address(serv_info.unwrap());
        //ip.first().unwrap().to_owned()

    }
}
