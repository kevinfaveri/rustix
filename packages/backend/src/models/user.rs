use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
  pub id: Uuid,
  pub username: String,
  pub password: String,
  #[serde(rename = "googleOAuth")]
  pub google_oauth: Option<String>,
  #[serde(rename = "appleOAuth")]
  pub apple_oauth: Option<String>,
  #[serde(rename = "discordOAuth")]
  pub discord_oauth: Option<String>,
  #[serde(rename = "githubOAuth")]
  pub user_prompt_persona: Option<String>,
  #[serde(rename = "createdAt")]
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: chrono::DateTime<chrono::Utc>,
}
