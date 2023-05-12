use crate::{
    connect::{
        address::{Address, AddressBuilder},
        discover::{ServiceProvider, SERVICE_TYPE},
        socket::{PeerMap, poll_client_connections, setup_server},
    },
    clipshare::{
        device::{Dev, init_device},
        settings::{Settings, SettingsBuilder},
    },
};

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
    collections::HashMap,
};

use local_ip_address::local_ip;

pub struct Server {
    pub settings: Settings,
    pub device: Dev,
    pub address: Arc<Address>,
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
    
    let device = ServiceProvider::new(service_type, instance_name, host_ipv4, &host_name, port, &properties[..]);
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
            AddressBuilder::default()
            .ip(local_ip().unwrap().to_string())
            .build().expect("Could not create AddressBuilder"));
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
