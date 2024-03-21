use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateUserSchema {
  #[validate(length(min = 8))]
  pub password: String,
  pub googleOauth: Option<String>,
  pub appleOauth: Option<String>,
  pub discord_oauth: Option<String>,
  #[validate(custom = "utils::validation::validate_user_prompt_persona")]
  pub userPromptPersona: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateUserSchema {
  #[validate(length(min = 8))]
  pub password: Option<String>,
  pub googleOauth: Option<String>,
  pub appleOauth: Option<String>,
  pub discordOauth: Option<String>,
  #[validate(custom = "utils::validation::validate_user_prompt_persona")]
  pub userPromptPersona: Option<String>,
}
