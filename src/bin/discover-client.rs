use clipshare::connect::discover::{SERVICE_TYPE, ServiceFinder};
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let device_monitor = ServiceFinder::new(SERVICE_TYPE);
    let timeout = Duration::from_secs(30);
    device_monitor.find_devices(timeout).await;
}
