use clipshare::discover::{SERVICE_TYPE, DeviceMonitor};

#[tokio::main]
pub async fn main() {
    let device_monitor = DeviceMonitor::new(SERVICE_TYPE);
    device_monitor.monitor_devices().await;
}
