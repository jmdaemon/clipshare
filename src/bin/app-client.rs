use clipshare::{
    clipshare::Client,
    ws::Address
};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let addr = client.monitor_devices().await;
    let address = Address::new(addr.to_string(), 5200);
    client.connect_to(address).await;
}
