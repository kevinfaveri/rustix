use serde::{Deserialize, Serialize};
use utils::validation;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateTripplanBoxSchema {
  #[validate(length(min = 1, max = 255))]
  pub title: String,
  #[validate(length(min = 1))]
  pub description: String,
  #[validate(length(equal = 36))]
  pub booked: bool,
  pub published: bool,
  pub publishedLink: Option<String>,
  #[validate(length(min = 1), custom = "utils::validation::validate_view_mode")]
  pub viewMode: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateTripplanBoxSchema {
  #[validate(length(min = 1, max = 255))]
  pub title: Option<String>,
  #[validate(length(min = 1))]
  pub description: Option<String>,
  pub booked: Option<bool>,
  pub published: Option<bool>,
  pub publishedLink: Option<String>,
  #[validate(length(min = 1), custom = "utils::validation::validate_view_mode")]
  pub viewMode: Option<String>,
}
