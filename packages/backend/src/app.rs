use std::str::FromStr;

use axum::{middleware, routing::get, Extension, Router};
use http::header;
use sqlx::{Pool, Postgres};
use tower_http::{
  compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
  sensitive_headers::SetSensitiveHeadersLayer, trace,
};
use tracing::Level;

use crate::{
  models::user::UserData,
  routes::{self, middlewares::inject_user_data},
  settings::SETTINGS,
  utils::auth::{login, logout, oauth_callback},
};

pub async fn create_app(db: Pool<Postgres>) -> Router {
  let user_data: Option<UserData> = None;
  Router::new()
    .merge(routes::status::create_router())
    .merge(routes::user::create_router())
    // High level logging of requests and responses
    .layer(
      trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().include_headers(false))
        .on_request(
          trace::DefaultOnRequest::new()
            .level(Level::from_str(SETTINGS.logger_level.as_str()).unwrap_or(Level::INFO)),
        )
        .on_response(
          trace::DefaultOnResponse::new()
            .level(Level::from_str(SETTINGS.logger_level.as_str()).unwrap_or(Level::INFO)),
        ),
    )
    // Mark the `Authorization` request header as sensitive so it doesn't
    // show in logs.
    .layer(SetSensitiveHeadersLayer::new(std::iter::once(
      header::AUTHORIZATION,
    )))
    // Compress responses
    .layer(CompressionLayer::new())
    // Propagate `X-Request-Id`s from requests to responses
    .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
      "x-request-id",
    )))
    .route("/login", get(login))
    .route("/oauth_callback", get(oauth_callback))
    .route("/logout", get(logout))
    .route_layer(middleware::from_fn(inject_user_data))
    // TODO: CORS configuration. This should probably be more restrictive in production
    .layer(CorsLayer::permissive())
    .layer(Extension(db))
    .layer(Extension(user_data))
}
