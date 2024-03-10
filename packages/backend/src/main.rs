use dotenvy::dotenv;
use std::env;
use utils::server::serve_app;

mod app;
mod errors;
mod logger;
mod settings;
mod utils;

use settings::SETTINGS;
#[tokio::main]
async fn main() {
  dotenv().expect(".env file not found");
  let app = app::create_app().await;
  for (key, value) in env::vars() {
    println!("{key}: {value}");
  }
  let port = SETTINGS.server_port;
  serve_app(app, port).await;
}
