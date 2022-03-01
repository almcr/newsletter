use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  run(TcpListener::bind("127.0.0.1:8080").expect("failed to bind"))?.await
}
