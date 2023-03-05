use clipshare::discover::{SERVICE_TYPE, discover_client_handle_event};
use mdns_sd::ServiceDaemon;

#[tokio::main]
pub async fn main() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let receiver = mdns.browse(SERVICE_TYPE).expect("Failed to browse");

    while let Ok(event) = receiver.recv_async().await {
        discover_client_handle_event(&event);
    }
}
