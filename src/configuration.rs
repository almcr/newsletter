use secrecy::{ExposeSecret, Secret, SecretString};

#[derive(serde::Deserialize)]
pub struct Settings {
  pub app_port: u16,
  pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub port: u16,
  pub host: String,
  pub username: String,
  pub password: SecretString,
  pub db_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let mut settings = config::Config::default();

  settings.merge(config::File::with_name("configuration"))?;

  settings.try_into()
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> SecretString {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port,
      self.db_name
    ))
  }

  pub fn connection_string_without_db(&self) -> SecretString {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port
    ))
  }
}
