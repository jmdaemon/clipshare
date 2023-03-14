use clipshare::{
    discover::{DeviceMonitor, handle_new_client, SERVICE_TYPE},
    device::init_device,
    ws::setup_client,
    config::Settings,
};
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex}
};

#[tokio::main]
async fn main() {
    // Load the clipboard for the current device
    let _settings = Settings::default().load_config();
    let dev = init_device();
    let device = Arc::new(Mutex::new(dev));

    let mut addr = Ipv4Addr::LOCALHOST;
    let port = 5200;

    // Wait for device to be paired
    let device_monitor = DeviceMonitor::new(SERVICE_TYPE);
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
    let url = format!("ws://{}:{}", addr, port);
    //let client = setup_client(url);
    let client = setup_client(device, url);
    tokio::join!(client);
}
