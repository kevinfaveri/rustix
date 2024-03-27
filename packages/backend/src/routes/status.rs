use crate::utils::json::json_wrapper;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
  Router::new().route("/status", get(json_wrapper("success")))
}
