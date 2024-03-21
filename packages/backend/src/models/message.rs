use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MessageModel {
  pub id: Uuid,
  pub content: serde_json::Value,
  #[serde(rename = "layoutContent")]
  pub layout_content: serde_json::Value,
  #[serde(rename = "tripplanboxId")]
  pub trippplanbox_id: Uuid,
  #[serde(rename = "createdAt")]
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: chrono::DateTime<chrono::Utc>,
}
