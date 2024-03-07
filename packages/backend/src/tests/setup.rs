use bson::doc;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

use crate::app::create_app;
use crate::models::cat::Cat;
use crate::models::user::User;
use crate::settings::SETTINGS;
use crate::utils::models::ModelExt;
use crate::utils::server::serve_app;

static API: OnceCell<()> = OnceCell::const_new();

pub async fn start_api_once() {
  API
    .get_or_init(|| async {
      std::env::set_var("RUN_MODE", "test");

      let app = create_app().await;
      let port = SETTINGS.server.port;

      tokio::spawn(async move {
        serve_app(app, port).await;
      });
    })
    .await;
}

lazy_static! {
  static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

pub fn use_app<F>(test: F)
where
  F: std::future::Future,
{
  RUNTIME.block_on(async move {
    start_api_once().await;

    Cat::delete_many(doc! {}).await.unwrap();
    User::delete_many(doc! {}).await.unwrap();

    test.await;
  })
}
