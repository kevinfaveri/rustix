use lazy_static::lazy_static;
use std::env;

lazy_static! {
  pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Debug)]
pub struct Settings {
  pub environment: String,
  pub server_port: u16,
  pub database_uri: String,
  pub database_name: String,
  pub auth_secret: String,
  pub logger_level: String,
}

impl Settings {
  pub fn new() -> Result<Self, env::VarError> {
    Ok(Self {
      environment: env::var("ENVIRONMENT")?,
      server_port: env::var("SERVER_PORT")?
        .parse()
        .expect("SERVER_PORT must be a number"),
      database_uri: env::var("DATABASE_URI")?,
      database_name: env::var("DATABASE_NAME")?,
      auth_secret: env::var("AUTH_SECRET")?,
      logger_level: env::var("LOGGER_LEVEL")?,
    })
  }
}
