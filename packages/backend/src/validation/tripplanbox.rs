use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct CreateTripplanBoxSchema {
  #[validate(length(min = 1, max = 255))]
  pub title: String,
  #[validate(length(min = 1))]
  pub description: String,
  pub booked: bool,
  pub published: bool,
  pub publishedLink: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_view_mode"))]
  pub viewMode: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[allow(non_snake_case)]
pub struct UpdateTripplanBoxSchema {
  #[validate(length(min = 1, max = 255))]
  pub title: Option<String>,
  #[validate(length(min = 1))]
  pub description: Option<String>,
  pub booked: Option<bool>,
  pub published: Option<bool>,
  pub publishedLink: Option<String>,
  #[validate(custom(function = "crate::utils::validators::validate_view_mode"))]
  pub viewMode: Option<String>,
}
