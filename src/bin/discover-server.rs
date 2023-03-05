use clipshare::discover::server_create_service;
use mdns_sd::ServiceDaemon;

pub fn main() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let my_service = server_create_service().unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service).expect("Failed to register our service");
}
