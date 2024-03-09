use utils::server::serve_app;

mod app;
mod errors;
mod logger;
mod settings;
mod utils;

use settings::SETTINGS;
#[tokio::main]
async fn main() {
  let app = app::create_app().await;

  let port = SETTINGS.server.port;
  serve_app(app, port).await;
}
