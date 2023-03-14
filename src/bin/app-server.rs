use clipshare::clipshare::Server;

#[tokio::main]
pub async fn main() {
    let server = Server::new();
    server.start().await;
}
