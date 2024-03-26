use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct CreateMessageSchema {
  pub content: serde_json::Value,
  pub layoutContent: serde_json::Value,
  #[validate(custom(function = "crate::utils::validators::validate_uuid_v4"))]
  pub tripplanboxId: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct UpdateMessageSchema {
  pub content: Option<serde_json::Value>,
  pub layoutContent: Option<serde_json::Value>,
}
