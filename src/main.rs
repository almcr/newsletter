
use newsletter::configuration::get_configuration; 
use newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;



#[tokio::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("name".into(), "info".into(), std::io::stdout);
  init_subscriber(subscriber);

  let settings = get_configuration().expect("Faield to parse configuration");
  let connection = PgPool::connect(&settings.database.connection_string_without_db().expose_secret())
    .await.
    .expect("Failed to connect to postgres");

  let listener =
    TcpListener::bind(format!("localhost:{}", settings.app_port)).expect("failed to bind");
  newsletter::startup::run(listener, connection)?.await
}
