use mdns_sd::{ServiceEvent, ServiceInfo};

pub const SERVICE_TYPE: &str = "_clipshare._udp.local.";

pub fn server_create_service() -> Result<ServiceInfo, mdns_sd::Error> {
    // Create a service info.
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    let host_ipv4 = "192.168.1.12";
    let host_name = "192.168.1.12.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        host_ipv4,
        port,
        &properties[..],
    )
}

pub fn client_handle_event(event: &ServiceEvent) {
    match event {
        ServiceEvent::ServiceResolved(info) => {
            println!("Resolved a new service: {}", info.get_fullname());
        }
        other_event => {
            println!("Received other event: {:?}", &other_event);
        }
    }
}
