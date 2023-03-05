use mdns_sd::{ServiceDaemon, ServiceEvent};

#[tokio::main]
pub async fn main() {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_mdns-sd-my-test._udp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    //std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("Resolved a new service: {}", info.get_fullname());
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    //});
}
