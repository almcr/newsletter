use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use std::net::TcpListener;
use newsletter::configuration::{get_configuration, DatabaseSettings};
use newsletter::telemetry::{get_subscriber, init_subscriber};
use uuid::Uuid;


// Ensure that the `tracing` stack is only initialized once
static TRACING: Lazy<()> = Lazy::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "debug".to_string();
  
  if std::env::var("TEST_LOG").is_ok() {
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
    init_subscriber(subscriber);
  } else {
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
    init_subscriber(subscriber);
  }
});

struct TestApp {
  pub pgpool: PgPool,
  pub address: String,
}

async fn spawn_app() -> TestApp {
  // executed only once 
  Lazy::force(&TRACING);

  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", &port);

  let mut config = get_configuration().expect("Failed to read configuration");
  config.database.db_name = Uuid::new_v4().to_string();
  let connection_pool = configure_database(&config.database).await;

  let server = newsletter::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  TestApp {
    address,
    pgpool: connection_pool
  }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
    .await
    .expect("Failed to connect to postgres");
  
  // Create Dabtabse
  connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
    .await
    .expect("Failed to create database");

  // Migrate Database
  let connection_pool = PgPool::connect(&config.connection_string_without_db().expose_secret())
    .await
    .expect("Failed to connect to postgres.");
  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");
  connection_pool
}

#[tokio::test]
async fn health_check_works() {
  let app = spawn_app().await;

  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", &app.address))
    .send()
    .await
    .expect("Failed to send request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_200_when_valid_dataform() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let body = "name=Ali%20MECERHED&email=mcrhd.ali%40gmail.com";

  let response = client
    .post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to send request");

  assert_eq!(200, response.status().as_u16());
  
  let saved = sqlx::query!("SELECT email, name FROM subscriptions")
    .fetch_one(&app.pgpool)
    .await
    .expect("Failed to fetch saved subscription");

  assert_eq!(saved.email, "mcrhd.ali@gmail.com");
  assert_eq!(saved.name, "Ali MECERHED");
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
  let app = spawn_app().await;

  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  for (invalide_body, error_message) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalide_body)
      .send()
      .await
      .expect("Failed to execute request.");

    assert_eq!(
      400,
      response.status().as_u16(),
      // Additional customised error message on test failure
      "The API did not fail with 400 Bad Request when the payload was {}.",
      error_message
    );
  }
}
