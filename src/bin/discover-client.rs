use clipshare::connect::discover::{SERVICE_TYPE, ServiceFinder};

#[tokio::main]
pub async fn main() {
    let device_monitor = ServiceFinder::new(SERVICE_TYPE);
    device_monitor.monitor_devices().await;
}
