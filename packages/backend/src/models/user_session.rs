use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSessionModel {
  pub id: Uuid,
  pub user_id: Uuid,
  pub session_token_p1: String,
  pub session_token_p2: String,
  pub oauth_provider: String,
  pub expires_at: NaiveDateTime,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
