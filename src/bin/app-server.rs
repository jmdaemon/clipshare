use clipshare::connect::server::Server;

#[tokio::main]
pub async fn main() {
    let server = Server::new();
    server.start().await;
}
