use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
  pub id: Uuid,
  pub username: String,
  pub password: String,
  pub google_oauth: Option<String>,
  pub apple_oauth: Option<String>,
  pub discord_oauth: Option<String>,
  pub user_prompt_persona: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
