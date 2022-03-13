use sqlx::{Connection, PgConnection, Executor};
use std::net::TcpListener;

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let server = newsletter::startup::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);
  format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
  let address = spawn_app();

  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to send request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_200_when_valid_dataform() {
  let address = spawn_app();
  let config = newsletter::configuration::get_configuration().expect("Failed to get configuration");

  let mut pg_connection = PgConnection::connect(&config.database.connection_string())
    .await
    .expect("Failed to connect to postgres");
  

  let client = reqwest::Client::new();

  let body = "name=Ali%20MECERHED&email=mcrhd.ali%40gmail.com";
  let response = client
    .post(&format!("{}/subscriptions", &address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to send request");

  assert_eq!(200, response.status().as_u16());
  
  let saved = sqlx::query!("SELECT email, name FROM subscriptions")
    .fetch_one(&mut pg_connection)
    .await
    .expect("Failed to fetch saved data");

  assert_eq!(saved.email, "ursula_le_guin@gmail.com");
  assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
  let app_address = spawn_app();

  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  for (invalide_body, error_message) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &app_address))
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
