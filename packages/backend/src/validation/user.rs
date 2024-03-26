use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct CreateUserSchema {
  #[validate(email)]
  pub username: String,
  #[validate(length(min = 8))]
  pub password: Option<String>,
  pub googleOauth: Option<String>,
  pub appleOauth: Option<String>,
  pub discord_oauth: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_user_prompt_persona"))]
  pub userPromptPersona: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct UpdateUserSchema {
  #[validate(length(min = 8))]
  pub password: Option<String>,
  pub googleOauth: Option<String>,
  pub appleOauth: Option<String>,
  pub discordOauth: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_user_prompt_persona"))]
  pub userPromptPersona: Option<String>,
}
