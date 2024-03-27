use axum::{
  extract::{Extension, Query},
  response::IntoResponse,
  routing::get,
  Router,
};
use axum_extra::TypedHeader;
use headers::Cookie;
use oauth2::{
  reqwest::http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope,
  TokenResponse,
};

use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
  error::Error,
  models::user::UserData,
  repositories::{
    oauth2_state_storage::{delete_oauth2_state_storage, insert_oauth2_state_storage},
    user::{create_user, get_user},
    user_sessions::{create_user_session, delete_user_session},
  },
  utils::{auth::get_client, json::json_wrapper},
  validation::user::CreateUserSchema,
};

pub async fn login(
  user_data: Extension<Option<UserData>>,
  db: Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
  if user_data.is_some() {
    return Err(Error::AlreadyAuthenticated);
  }

  let client = get_client()?;

  let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

  let (authorize_url, csrf_state) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new(
      "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ))
    .add_scope(Scope::new(
      "https://www.googleapis.com/auth/userinfo.profile".to_string(),
    ))
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  insert_oauth2_state_storage(db.clone(), &csrf_state, &pkce_code_verifier).await?;

  Ok(json_wrapper(authorize_url.to_string()))
}

pub async fn oauth_callback(
  Query(mut params): Query<HashMap<String, String>>,
  db: Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
  let state = CsrfToken::new(params.remove("state").expect("OAuth: without state"));
  let code = AuthorizationCode::new(params.remove("code").expect("OAuth: without code"));

  let oauth2_state = delete_oauth2_state_storage(db.clone(), &state).await?;
  let pkce_code_verifier = PkceCodeVerifier::new(oauth2_state.pkce_code_verifier);

  // Exchange the code with a token.
  let client = get_client()?;
  let token_response = tokio::task::spawn_blocking(move || {
    client
      .exchange_code(code)
      .set_pkce_verifier(pkce_code_verifier)
      .request(http_client)
  })
  .await
  .expect("OAuth: exchange_code failure")
  .expect("OAuth: tokio spawn blocking failure");
  let access_token = token_response.access_token().secret();

  // Get user info from Google
  let url = "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=".to_owned() + access_token;
  let body = reqwest::get(url)
    .await
    .expect("OAuth: reqwest failed to query userinfo")
    .text()
    .await
    .expect("OAuth: reqwest received invalid userinfo");
  let mut body: serde_json::Value =
    serde_json::from_str(body.as_str()).expect("OAuth: Serde failed to parse userinfo");
  let email = body["email"]
    .take()
    .as_str()
    .expect("OAuth: Serde failed to parse email address")
    .to_owned();
  let verified_email: bool = body["verified_email"]
    .take()
    .as_bool()
    .expect("OAuth: Serde failed to parse verified_email");
  if !verified_email {
    return Err(Error::from(anyhow::anyhow!("Your email address is not verified. Please verify your email address with Google and try again.")));
  }
  let user_name = body["name"].as_str().map(|s| s.to_owned());
  let user_avatar = body["picture"].as_str().map(|s| s.to_owned());

  // Check if user exists in database
  // If not, create a new user
  let user = get_user(db.clone(), email.clone()).await;
  let user_id = if let Ok(user) = user {
    user.id
  } else {
    let saved_user = create_user(
      db.clone(),
      CreateUserSchema {
        username: email.clone(),
        user_name,
        user_avatar,
        user_prompt_persona: Some("alfred".to_owned()),
      },
    )
    .await?;
    saved_user.id
  };

  // Create a session for the user
  let session_token_p1 = Uuid::new_v4().to_string();
  let session_token_p2 = Uuid::new_v4().to_string();
  let session_token = [session_token_p1.as_str(), "_", session_token_p2.as_str()].concat();

  let user_session = create_user_session(
    db.clone(),
    &session_token_p1,
    &session_token_p2,
    &user_id,
    &"google".to_string().to_owned(),
  )
  .await?;

  let expires_str = user_session
    .expires_at
    .format("%a, %d %b %Y %H:%M:%S GMT")
    .to_string();

  let headers = axum::response::AppendHeaders([(
    axum::http::header::SET_COOKIE,
    "session_token=".to_owned()
      + &*session_token
      + "; path=/; httponly; secure; samesite=strict; expires="
      + &expires_str,
  )]);

  Ok((headers, json_wrapper("success")))
}

pub async fn logout(
  cookie: Option<TypedHeader<Cookie>>,
  db: Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
  if let Some(cookie) = cookie {
    if let Some(session_token) = cookie.get("session_token") {
      let session_token: Vec<&str> = session_token.split('_').collect();
      delete_user_session(db, &session_token[0].to_string().to_owned()).await?;
    }
  }
  let headers = axum::response::AppendHeaders([(
    axum::http::header::SET_COOKIE,
    "session_token=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
  )]);
  Ok((headers, json_wrapper("success")))
}

pub fn create_router() -> Router {
  Router::new()
    .route("/login", get(login))
    .route("/oauth_callback", get(oauth_callback))
    .route("/logout", get(logout))
}
