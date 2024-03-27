use std::str::FromStr;

use axum::{middleware, Extension, Router};
use http::header;
use sqlx::{Pool, Postgres};
use tower_http::{
  compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
  sensitive_headers::SetSensitiveHeadersLayer, trace,
};
use tracing::Level;

use crate::{middlewares::inject_user_data, models::user::UserData, routes, settings::SETTINGS};

pub async fn create_app(db: Pool<Postgres>) -> Router {
  let origins = [
    "http://localhost:3000".parse().unwrap(),
    "http://localhost:8080".parse().unwrap(),
    "https://trippp.ai".parse().unwrap(),
  ];
  let user_data: Option<UserData> = None;
  Router::new()
    .merge(routes::status::create_router())
    .merge(routes::user::create_router())
    .merge(routes::auth::create_router())
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
    .layer(SetSensitiveHeadersLayer::new(std::iter::once(
      header::AUTHORIZATION,
    )))
    .layer(SetSensitiveHeadersLayer::new(std::iter::once(
      header::COOKIE,
    )))
    // Compress responses
    .layer(CompressionLayer::new())
    // Propagate `X-Request-Id`s from requests to responses
    .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
      "x-request-id",
    )))
    .route_layer(middleware::from_fn(inject_user_data::get_middleware_fn))
    .layer(CorsLayer::new().allow_origin(origins))
    .layer(Extension(db))
    .layer(Extension(user_data))
}
