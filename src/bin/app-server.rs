use clipshare::ws::{setup_server, poll_client_connections};
use clipshare::ws::PeerMap;
use clipshare::discover::{Device, DeviceMonitor, SERVICE_TYPE, handle_new_client};

use std::{
    thread::sleep,
    time::Duration,
    env,
    sync::Mutex,
    collections::HashMap,
};

pub async fn start_server() {
    // Create server
    //let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //let addr = env::args().nth(1).unwrap_or_else(|| "192.168.1.12:8080".to_string());
    //let addr = env::args().nth(1).unwrap_or_else(|| "192.168.1.76:8080".to_string());
    let addr = env::args().nth(1).unwrap_or_else(|| "192.168.1.76:5200".to_string());
    let server = setup_server(addr.clone());
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    tokio::join!(poll_client_connections(server.await, state));
}

pub async fn register_device() {
    // Attempt to register device
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    //let host_ipv4 = "192.168.1.12";
    //let host_name = "192.168.1.12.local.";
    let host_ipv4 = "192.168.1.76";
    let host_name = "192.168.1.76.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];
    
    let device = Device::new(service_type, instance_name, host_ipv4, host_name, port, &properties[..]);
    //device.register_device().await;
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
    let thread_register_device = tokio::spawn(async {
        register_device().await
    });
    let thread_start_server = tokio::spawn(async {
        start_server().await
    });
    // In another thread, establish the server
    tokio::join!(
        thread_register_device,
        thread_start_server,
    );
}
