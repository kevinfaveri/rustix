use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TripplanBoxModel {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  #[serde(rename = "userId")]
  pub user_id: Uuid,
  pub booked: bool,
  pub published: bool,
  #[serde(rename = "publishedLink")]
  pub published_link: Option<String>,
  #[serde(rename = "viewMode")]
  pub view_mode: String,
  #[serde(rename = "createdAt")]
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: chrono::DateTime<chrono::Utc>,
}
