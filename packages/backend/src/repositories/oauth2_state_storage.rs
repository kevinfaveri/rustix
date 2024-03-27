use axum::Extension;
use oauth2::{CsrfToken, PkceCodeVerifier};
use sqlx::PgPool;

use crate::models::oauth2_state_storage::OAuth2StateStorageModel;

pub async fn insert_oauth2_state_storage(
  db: Extension<PgPool>,
  csrf_state: &CsrfToken,
  pkce_code_verifier: &PkceCodeVerifier,
) -> Result<OAuth2StateStorageModel, sqlx::Error> {
  let res = sqlx::query_as!(
    OAuth2StateStorageModel,
    // language=PostgreSQL
    r#"
      INSERT INTO oauth2_state_storage (csrf_state, pkce_code_verifier) 
      VALUES ($1, $2)
      RETURNING id, csrf_state, pkce_code_verifier
    "#,
    csrf_state.secret(),
    pkce_code_verifier.secret(),
  )
  .fetch_one(&*db)
  .await?;

  Ok(res)
}

pub async fn delete_oauth2_state_storage(
  db: Extension<PgPool>,
  state: &CsrfToken,
) -> Result<OAuth2StateStorageModel, sqlx::Error> {
  let res = sqlx::query_as!(
    OAuth2StateStorageModel,
    // language=PostgreSQL
    r#"
      DELETE FROM oauth2_state_storage WHERE csrf_state = $1 RETURNING id, csrf_state, pkce_code_verifier
    "#,
    state.secret(),
  )
  .fetch_one(&*db)
  .await?;

  Ok(res)
}
