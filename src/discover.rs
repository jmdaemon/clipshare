use std::{
    io::{Write, Read},
    time::Duration,
    thread::sleep,
    net::{TcpStream, TcpListener},
    process::exit,
};
use mdns_sd::{ServiceEvent, ServiceInfo, ServiceDaemon, Receiver};

pub const SERVICE_TYPE: &str = "_clipshare._udp.local.";

pub enum DeviceMsg {
    RegisterDevice,
    DeviceRegistered,
}

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

fn handle_new_client(event: &ServiceEvent) {
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

            // Tell the device to stop sending connection messages
            addresses.iter().for_each(|addr| {
                //let address = format!("{}:9124", addr);
                //let address = format!("{}:5201", addr);
                let address = format!("{}:5201", "192.168.1.14");
                //let mut stream = TcpStream::connect(addr.to_string())
                let mut stream = TcpStream::connect(address)
                    .expect("Couldn't connect to device...");
                stream.write_all(&[1]).unwrap();
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            });
                    }
        other_event => {
            println!("Received other event: {:?}", &other_event);
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

            let addresses = service.get_addresses();
            for addr in addresses {
                //println!("{}", addr);
                //let address = format!("{}:5000", addr.to_string());
                //let address = format!("{}:9124", addr);
                //let address = format!("{}:5201", addr);
                let address = format!("{}:5201", "192.168.1.14");
                let listener = TcpListener::bind(address).unwrap();
                //let mut stream = TcpStream::connect(addr.to_string())
                        //.expect("Couldn't connect to device...");
                loop {
                    // Register with the daemon, which publishes the service.
                    self.mdns.register(service.to_owned()).expect("Failed to register our service");
                    for stream in listener.incoming() {
                        let mut stream = stream.unwrap();
                        let mut buf = [0; 1];
                        let result = stream.read(&mut buf).unwrap();
                        if result == 1 { // Shutdown signal
                            exit(0);
                        }
                    }
                    //let result = stream.read_timeout(Duration::from_secs(15));
                    //let mut buf = [0; 1];
                    //let result = stream.read(&mut buf).unwrap();
                    //if result == 1 { // Shutdown signal
                        //exit(0);
                    //}
                }
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
