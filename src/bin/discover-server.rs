use clipshare::discover::{Device, SERVICE_TYPE};

#[tokio::main]
pub async fn main() {
    let service_type = SERVICE_TYPE;
    let instance_name = "my_instance";
    let host_ipv4 = "192.168.1.12";
    let host_name = "192.168.1.12.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];
    
    let device = Device::new(service_type, instance_name, host_ipv4, host_name, port, &properties[..]);
    device.register_device().await;
}
