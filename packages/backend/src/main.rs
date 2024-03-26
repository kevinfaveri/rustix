use dotenvy::dotenv;

mod app;
mod database;
mod error;
mod logger;
mod models;
mod repositories;
mod routes;
mod settings;
mod utils;
mod validation;

#[tokio::main]
async fn main() {
  dotenv().expect(".env file not found");
  logger::setup();
  let db = database::start_db()
    .await
    .expect("Failed to start database");
  let app: axum::Router = app::create_app(db).await;
  let port = settings::SETTINGS.server_port;
  utils::server::serve_app(app, port).await;
}
