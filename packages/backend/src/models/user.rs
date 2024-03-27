use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
  pub id: Uuid,
  pub username: String,
  pub user_name: Option<String>,
  pub user_avatar: Option<String>,
  pub user_prompt_persona: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
  pub id: Uuid,
  pub username: String,
  pub user_name: Option<String>,
  pub user_avatar: Option<String>,
  pub user_prompt_persona: Option<String>,
  pub current_session_expire_at: i64,
}
