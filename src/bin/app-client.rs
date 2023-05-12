use clipshare::connect::{
    client::Client,
    address::AddressBuilder,
};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let addresses = client.get_device_addresses().await;
    let addr = addresses.get(0).expect("Unable to retrieve any device address");
    let address = AddressBuilder::default()
        .ip(addr.to_string())
        .build()
        .expect("Could not create AddressBuilder");
    client.connect_to(address).await;
}
