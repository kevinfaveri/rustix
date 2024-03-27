use axum::{body::Body, extract::Request, middleware::Next, response::IntoResponse, Extension};
use axum_extra::TypedHeader;
use chrono::Utc;
use headers::Cookie;
use sqlx::PgPool;
use tracing::debug;

use crate::{error::Error, models::user::UserData, routes::auth::logout};

pub async fn get_middleware_fn(
  cookie: Option<TypedHeader<Cookie>>,
  Extension(user_data): Extension<Option<UserData>>,
  request: Request<Body>,
  next: Next,
) -> Result<impl IntoResponse, Error> {
  debug!("Checking auth...");
  if let Some(user_data) = user_data {
    // Add check for expiration time; if expired, call logout to permanently remove session and return unauthorized
    if user_data.current_session_expire_at < Utc::now().timestamp() {
      logout(
        // Pass the session_token as an argument to the logout function
        cookie,
        Extension(request.extensions().get::<PgPool>().unwrap().clone()),
      )
      .await?;
      return Err(Error::Unauthorized);
    }
    debug!("Logged User: {:?}", user_data.username);
    Ok(next.run(request).await)
  } else {
    Err(Error::Unauthorized)
  }
}
