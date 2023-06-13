use std::{
    sync::{Arc, Mutex},
    time::Duration,
    thread::sleep,
};
use mdns_sd::{ServiceEvent, ServiceInfo, ServiceDaemon, Receiver};
use tokio::time::timeout;

/*
 * These are temporary strutures to discover devices via mdns-sd
 */

pub const SERVICE_TYPE: &str = "_clipshare._udp.local.";
pub type DeviceInfoPool = Arc<Mutex<Vec<ServiceInfo>>>;

pub struct ServiceFinder {
    pub mdns: ServiceDaemon,
    pub receiver: Receiver<ServiceEvent>,
}

pub struct ServiceProvider {
    pub mdns: ServiceDaemon,
    pub service_info: Option<ServiceInfo>,
}

fn get_service_info(event: &ServiceEvent) -> Option<ServiceInfo> {
    match event {
        ServiceEvent::ServiceResolved(info) => {
            println!("Resolved a new service: {}", info.get_fullname());

            // Show addresses
            let addresses = info.get_addresses();
            addresses.iter().for_each(|addr| {
                println!("Address Found: {}", addr);
            });

            // Show address properties
            info.get_properties().iter().for_each(|p| {
                println!("Property Name {}: Property Value {}", p.key(), p.val());
            });

            // Return the ServiceInfo file
            Some(info.to_owned())
        }
        other_event => {
            println!("Received other event: {:?}", &other_event);
            None
        }
    }
}

impl ServiceFinder {
    pub fn new(service_type: &str) -> ServiceFinder {
        // Create a daemon
        let mdns = ServiceDaemon::new().expect("Failed to create daemon");
        
        // Browse for a service type.
        let receiver: Receiver<ServiceEvent> = mdns.browse(service_type).expect("Failed to browse");
        ServiceFinder { mdns, receiver }
    }

    async fn recv_timeout(&self, time: Duration) -> Option<ServiceInfo> {
        while let Ok(received) = timeout(time, self.receiver.recv_async()).await {
            if let Ok(event) = received {
                if let Some(info) = get_service_info(&event) {
                    return Some(info);
                }
            }
        }
        None
    }

    /// Finds the first the device running the service
    /// Timeouts after a given duration
    pub async fn find_device(&self, time: Duration) -> Option<ServiceInfo> {
        self.recv_timeout(time).await
    }

    /*
    /// Asynchronously finds all the devices running the service
    /// Any devices found will be added to the DeviceInfoPool
    /// Automatically timeouts after a given duration
    pub async fn find_devices(&self, time: Duration, device_info_pool: DeviceInfoPool) {
        /*
        let mut devices = vec![];
        while let Ok(received) = timeout(time, self.receiver.recv_async()).await {
            if let Ok(event) = received {
                if let Some(info) = get_service_info(&event) {
                    device_info_pool.lock().expect("Unable to lock device_info_pool").push(info);
                }
            }
        }
    }
        */
        while let Some(info) = self.recv_timeout(time).await {
            device_info_pool.lock().expect("Unable to lock device_info_pool").push(info);
        }
    }
*/
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

impl ServiceProvider {

    pub fn new(
        service_type: &str,
        instance_name: &str,
        host_ipv4: &str,
        host_name: &str,
        port: u16,
        properties: &[(&str,&str)]
    ) -> ServiceProvider {

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
        ServiceProvider { mdns, service_info: Some(service_info) }
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
