use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let settings = get_configuration().expect("Faield to parse configuration");
  run(TcpListener::bind(format!("localhost:{}", settings.app_port)).expect("failed to bind"))?.await
}
