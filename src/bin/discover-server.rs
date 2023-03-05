use clipshare::discover::{SERVICE_TYPE};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

#[tokio::main]
pub async fn main() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    let host_ipv4 = "192.168.1.12";
    let host_name = "192.168.1.12.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        host_ipv4,
        port,
        &properties[..],
    ).unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service).expect("Failed to register our service");
    //while mdns.monitor() {
        //mdns
    //}
    //mdns.monitor();
}
