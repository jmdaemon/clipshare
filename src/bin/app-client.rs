use clipshare::{
    connect::{
        client::Client,
        address::AddressBuilder,
    }
};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let addr = client.monitor_devices().await;
    let address = AddressBuilder::new()
        .ip(addr.to_string())
        .build();
    client.connect_to(address).await;
}
