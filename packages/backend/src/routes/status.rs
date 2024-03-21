use crate::error::Error;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;

async fn get_status() -> Result<Json<Status>, Error> {
  debug!("Returning status");
  Ok(Json(Status {
    status: "ok".to_owned(),
  }))
}

pub fn create_router() -> Router {
  Router::new().route("/status", get(get_status))
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
  status: String,
}
