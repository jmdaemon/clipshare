use std::{
    io::{Write, Read},
    time::Duration,
    thread::sleep,
    net::{TcpStream, TcpListener, Ipv4Addr},
    process::exit,
};
use mdns_sd::{ServiceEvent, ServiceInfo, ServiceDaemon, Receiver};

pub const SERVICE_TYPE: &str = "_clipshare._udp.local.";

pub struct DeviceMonitor {
    pub mdns: ServiceDaemon,
    receiver: Receiver<ServiceEvent>,
}

pub struct Device {
    pub mdns: ServiceDaemon,
    pub service_info: Option<ServiceInfo>,
}

impl DeviceMonitor {
    pub fn new(service_type: &str) -> DeviceMonitor {
        // Create a daemon
        let mdns = ServiceDaemon::new().expect("Failed to create daemon");
        
        // Browse for a service type.
        let receiver: Receiver<ServiceEvent> = mdns.browse(service_type).expect("Failed to browse");
        DeviceMonitor { mdns, receiver }
    }

    pub async fn monitor_devices(&self) {
        while let Ok(event) = self.receiver.recv_async().await {
            handle_new_client(&event);
        }
    }
}

fn handle_new_client(event: &ServiceEvent) -> Option<ServiceInfo> {
    match event {
        ServiceEvent::ServiceResolved(info) => {

            println!("Resolved a new service: {}", info.get_fullname());
            let addresses = info.get_addresses();
            addresses.iter().for_each(|addr| {
                println!("Address Found: {}", addr);
            });
            info.get_properties().iter().for_each(|p| {
                println!("{}: {}", p.key(), p.val());
            });

            // TODO: Tell the device to stop sending connection messages
            //addresses.iter().for_each(|addr| {
            //});
            Some(info.to_owned())
        }
        other_event => {
            println!("Received other event: {:?}", &other_event);
            None
        }
    }
}

impl Device {
    pub fn new(
        service_type: &str,
        instance_name: &str,
        host_ipv4: &str,
        host_name: &str,
        port: u16,
        properties: &[(&str,&str)]
    ) -> Device {

        // Create a daemon
        let mdns = ServiceDaemon::new().expect("Failed to create daemon");

        // Create a service info.
        let service_info = ServiceInfo::new(
            service_type,
            instance_name,
            host_name,
            host_ipv4,
            port,
            properties,
        ).unwrap();
        Device { mdns, service_info: Some(service_info) }
    }

    pub async fn register_device(&self) {
        if let Some(service) = &self.service_info {
            // TODO: Run forever for now, we'll kill this later somehow,
            // when we drop the connection.
            loop {
                // Register with the daemon, which publishes the service.
                self.mdns.register(service.to_owned()).expect("Failed to register our service");
                sleep(Duration::from_secs(15));
            }
        }
    }
}

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
