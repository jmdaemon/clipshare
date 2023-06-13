use clipshare::connect::{
    client::Client,
    address::AddressBuilder,
};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let address = client.get_device_address().await;
    let address = AddressBuilder::default()
        .ip(address.to_string())
        .build()
        .expect("Could not create AddressBuilder");
    client.connect_to(address).await;
}
