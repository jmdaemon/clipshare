use clipshare::{ws::setup_client, discover::{DeviceMonitor, handle_new_client, SERVICE_TYPE}};
use std::{env, net::Ipv4Addr};

#[tokio::main]
async fn main() {
    // Wait for device to be paired
    let device_monitor = DeviceMonitor::new(SERVICE_TYPE);

    let mut addr = Ipv4Addr::LOCALHOST;
    while let Ok(event) = device_monitor.receiver.recv_async().await {
        if let Some(service_info) = handle_new_client(&event) { 
            let addresses = service_info.get_addresses();
            addresses.iter().for_each(|address| {
                addr = address.to_owned();
            });
            if addr != Ipv4Addr::LOCALHOST {
                break;
            }
        }
    }
    println!("Connecting to device {}", addr);

    // Create client to the connected device
    //let url = format!("ws://{}", addr);
    //let url = format!("ws://{}:8080", addr);
    //let url = format!("ws://{}:8080", addr);
    //let url = format!("ws://{}.local:8080", addr);
    //let url = format!("ws://{}", addr);
    //let url = format!("ws://{}", addr);
    //let url = format!("ws://{}:8080", addr);
    let url = format!("ws://{}:5200", addr);
    //let url = format!("ws://127.0.0.1:8080");
    let client = setup_client(url);
    tokio::join!(client);
}
