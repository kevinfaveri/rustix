use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateMessageSchema {
  #[validate(length(min = 1))]
  pub content: serde_json::Value,
  #[validate(length(min = 1))]
  pub layoutContent: serde_json::Value,
  #[validate(length(equal = 36), custom = "utils::validation::validate_uuid_v4")]
  pub tripplanboxId: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateMessageSchema {
  pub content: Option<serde_json::Value>,
  pub layoutContent: Option<serde_json::Value>,
}
