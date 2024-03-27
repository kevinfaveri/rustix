use tracing::Level;

use crate::settings::SETTINGS;
use std::{env, str::FromStr};

pub fn setup() {
  let level = SETTINGS.logger_level.as_str();
  if env::var_os("RUST_LOG").is_none() {
    let app_name = env::var("CARGO_PKG_NAME").unwrap();
    let env = format!("{app_name}={level},tower_http={level},sqlx={level},middlewares={level}");

    env::set_var("RUST_LOG", env);
  }
  println!("RUST_LOG={}", env::var("RUST_LOG").unwrap_or_default());
  let level = Level::from_str(level).unwrap_or(Level::INFO);
  tracing_subscriber::fmt().with_max_level(level).init();
}
