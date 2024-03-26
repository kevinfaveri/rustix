use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TripplanBoxModel {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub user_id: Uuid,
  pub booked: bool,
  pub published: bool,
  pub published_link: Option<String>,
  pub view_mode: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
