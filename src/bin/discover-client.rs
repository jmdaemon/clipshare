//use futures::{pin_mut, stream::StreamExt};
//clipshare
//use std::{net::IpAddr, time::Duration};

////const SERVICE_NAME: &str = "_googlecast._tcp.local";
////const SERVICE_NAME: &str = "http._tcp.local";
////const SERVICE_NAME: &str = "http._tcp";
////const SERVICE_NAME: &str = "_systemd._tcp";
////const SERVICE_NAME: &str = "_gateway:bootps";
////const SERVICE_NAME: &str = "_http._tcp.local";
//const SERVICE_NAME: &str = "_http._tcp.local";

//#[tokio::main]
//async fn main() -> Result<(), Error> {
    //// Iterate through responses from each Cast device, asking for new devices every 15s
    ////let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    ////let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(0.001))?.listen();
    //let stream = mdns::discover::all(SERVICE_NAME, Duration::from_millis(1))?.listen();
    //pin_mut!(stream);

    //while let Some(Ok(response)) = stream.next().await {
        //let addr = response.ip_addr();
        ////let addr = response.records()
                           ////.filter_map(self::to_ip_addr)
                           ////.next();

        //if let Some(addr) = addr {
            //println!("found cast device at {}", addr);
        //} else {
            //println!("cast device does not advertise address");
        //}
    //}

    //Ok(())
//}

//fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    //match record.kind {
        //RecordKind::A(addr) => Some(addr.into()),
        //RecordKind::AAAA(addr) => Some(addr.into()),
        //_ => None,
    //}
//}

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
