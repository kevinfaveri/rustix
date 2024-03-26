use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageModel {
  pub id: Uuid,
  pub content: serde_json::Value,
  pub layout_content: serde_json::Value,
  pub trippplanbox_id: Uuid,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
