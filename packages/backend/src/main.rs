use dotenvy::dotenv;

mod app;
mod errors;
mod logger;
mod routes;
mod settings;
mod utils;

#[tokio::main]
async fn main() {
  dotenv().expect(".env file not found");
  logger::setup();
  let app = app::create_app().await;
  let port = settings::SETTINGS.server_port;
  utils::server::serve_app(app, port).await;
}
