use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse, Extension};
use axum_extra::TypedHeader;
use chrono::Utc;
use headers::Cookie;
use sqlx::PgPool;
use tracing::debug;

use crate::{
  error::Error,
  models::user::UserData,
  repositories::{user::get_user, user_sessions::get_user_session_by_token_p1},
  utils::auth::logout,
};

pub async fn inject_user_data(
  db: Extension<PgPool>,
  cookie: Option<TypedHeader<Cookie>>,
  mut request: Request<Body>,
  next: Next,
) -> Result<impl IntoResponse, Error> {
  if let Some(cookie) = cookie {
    if let Some(session_token) = cookie.get("session_token") {
      let session_token: Vec<&str> = session_token.split('_').collect();

      let user_session = get_user_session_by_token_p1(db.clone(), session_token[0]).await;

      if let Ok(user_session) = user_session {
        if let Ok(session_token_p2_db) = user_session.session_token_p2.as_bytes().try_into() {
          if let Ok(session_token_p2_cookie) = session_token
            .get(1)
            .copied()
            .unwrap_or_default()
            .as_bytes()
            .try_into()
          {
            if constant_time_eq::constant_time_eq_n::<36>(
              session_token_p2_cookie,
              session_token_p2_db,
            ) {
              let expires_at = user_session.expires_at.and_utc().timestamp();
              if expires_at > Utc::now().timestamp() {
                let user = get_user(db.clone(), user_session.user_id.to_string()).await;
                debug!("Logged User: {:?}", user);
                if let Ok(user) = user {
                  request.extensions_mut().insert(Some(UserData {
                    id: user.id,
                    username: user.username,
                    user_name: user.user_name,
                    user_avatar: user.user_avatar,
                    user_prompt_persona: user.user_prompt_persona,
                    current_session_expire_at: expires_at,
                  }));
                }
              }
            }
          }
        }
      }
    }
  }

  Ok(next.run(request).await)
}

pub async fn check_auth(
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
    debug!("Logged User: {:?}", user_data);
    Ok(next.run(request).await)
  } else {
    Err(Error::Unauthorized)
  }
}
