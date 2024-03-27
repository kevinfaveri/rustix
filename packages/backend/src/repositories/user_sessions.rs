use crate::{error::Error, models::user_session::UserSessionModel};
use axum::Extension;
use sqlx::PgPool;
use tracing::{debug, error};
use uuid::Uuid;

pub async fn get_user_session_by_token_p1(
  db: Extension<PgPool>,
  token_p1: &str,
) -> Result<UserSessionModel, Error> {
  debug!("Getting user session by token_p1...");
  match sqlx::query_as!(
    UserSessionModel,
    // language=PostgreSQL
    r#"
      SELECT id,user_id, session_token_p1, session_token_p2, expires_at, oauth_provider, created_at, updated_at
      FROM user_sessions
      WHERE session_token_p1=$1
    "#,
    token_p1
  )
  .fetch_one(&*db)
  .await
  {
    Ok(user_session) => Ok(user_session),
    Err(err) => {
      error!("Failed to get user session by token_p1: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn create_user_session(
  db: Extension<PgPool>,
  session_token_p1: &String,
  session_token_p2: &String,
  user_id: &Uuid,
  oauth_provider: &String,
) -> Result<UserSessionModel, Error> {
  let now = chrono::Utc::now().naive_utc();
  let expires_at = now + chrono::Duration::try_days(30).unwrap().to_std().unwrap();
  match sqlx::query_as!(
    UserSessionModel,
    // language=PostgreSQL
    r#"
      INSERT INTO user_sessions
        (session_token_p1, session_token_p2, user_id, oauth_provider, expires_at, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7)
      RETURNING id,user_id, session_token_p1, session_token_p2, expires_at, oauth_provider, created_at, updated_at
    "#,
    session_token_p1,
    session_token_p2,
    user_id,
    oauth_provider,
    expires_at,
    now,
    now
  )
  .fetch_one(&*db)
  .await
  {
    Ok(user_session) => Ok(user_session),
    Err(err) => {
      error!("Failed to create user session: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn delete_user_session(
  db: Extension<PgPool>,
  session_token_p1: &String,
) -> Result<UserSessionModel, Error> {
  match sqlx::query_as!(
    UserSessionModel,
    // language=PostgreSQL
    r#"
      DELETE FROM user_sessions
      WHERE session_token_p1 = $1
      RETURNING id,user_id, session_token_p1, session_token_p2, expires_at, oauth_provider, created_at, updated_at
    "#,
    session_token_p1,
  )
  .fetch_one(&*db)
  .await
  {
    Ok(user_session) => Ok(user_session),
    Err(err) => {
      error!("Failed to delete user session: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}
