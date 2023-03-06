use clipshare::ws::{Address, setup_server, poll_client_connections};
use clipshare::ws::PeerMap;
use clipshare::discover::{Device, SERVICE_TYPE};

use std::sync::Arc;
use std::{
    thread::sleep,
    time::Duration,
    sync::Mutex,
    collections::HashMap,
};
use local_ip_address::local_ip;

// Create server
pub async fn start_server(address: Arc<Address>) {
    let addr = address.to_string();
    let server = setup_server(addr.clone());
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    tokio::join!(poll_client_connections(server.await, state));
}

// Attempt to register device
pub async fn register_device(address: Arc<Address>) {
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    let addr = &address.ip;
    let host_ipv4 = addr;
    let host_name = format!("{}.local.", host_ipv4);
    let port = 5200;
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

#[tokio::main]
pub async fn main() {
    let ip = local_ip().unwrap().to_string();
    let port = 5200;

    let addr = Address { ip, port };
    let addr = Arc::new(addr);

    let addr_reg_dev = Arc::clone(&addr);
    let thread_register_device = tokio::spawn(async move {
        register_device(addr_reg_dev).await
    });

    let addr_start_serv = Arc::clone(&addr);
    let thread_start_server = tokio::spawn(async move {
        start_server(addr_start_serv).await
    });
    // In another thread, establish the server
    tokio::join!(
        thread_register_device,
        thread_start_server,
    );
}
