use crate::{error::Error, models::user_session::UserSessionModel};
use axum::Extension;
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

pub async fn get_user_session_by_token_p1(
  db: Extension<PgPool>,
  token_p1: &str,
) -> Result<UserSessionModel, Error> {
  debug!("Getting user session by token_p1...");
  let user_session = sqlx::query_as!(
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
  .await?;

  Ok(user_session)
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
  let user_session = sqlx::query_as!(
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
  .await?;

  Ok(user_session)
}

pub async fn delete_user_session(
  db: Extension<PgPool>,
  session_token_p1: &String,
) -> Result<(), Error> {
  sqlx::query!(
    // language=PostgreSQL
    r#"
      DELETE FROM user_sessions WHERE session_token_p1 = $1
    "#,
    session_token_p1,
  )
  .execute(&*db)
  .await?;

  Ok(())
}
