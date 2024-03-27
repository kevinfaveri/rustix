use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct CreateUserSchema {
  #[validate(email)]
  pub username: String,
  pub user_name: Option<String>,
  pub user_avatar: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_user_prompt_persona"))]
  pub user_prompt_persona: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct UpdateUserSchema {
  pub user_name: Option<String>,
  pub user_avatar: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_user_prompt_persona"))]
  pub userPromptPersona: Option<String>,
}
