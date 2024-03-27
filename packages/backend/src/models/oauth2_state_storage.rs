use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2StateStorageModel {
  pub id: Uuid,
  pub csrf_state: String,
  pub pkce_code_verifier: String,
}
