use mdns_sd::ServiceEvent;

pub const SERVICE_TYPE: &str = "_clipshare._udp.local.";

pub fn discover_client_handle_event(event: &ServiceEvent) {
    match event {
        ServiceEvent::ServiceResolved(info) => {
            println!("Resolved a new service: {}", info.get_fullname());
        }
        other_event => {
            println!("Received other event: {:?}", &other_event);
        }
    }
}


//pub fn discover_client_handle_event(event: &ServiceEvent) {
    //// TODO: Change 
    //discover_client_handle_event(&event);
//}
