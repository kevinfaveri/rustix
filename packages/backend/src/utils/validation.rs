use uuid::Uuid;
use validator::ValidationError;

#[allow(dead_code)]
pub fn validate_user_prompt_persona(persona: &str) -> Result<(), ValidationError> {
  match persona {
    "alfred" | "trippy" => Ok(()),
    _ => Err(ValidationError::new("invalid_user_prompt_persona")),
  }
}

#[allow(dead_code)]
pub fn validate_view_mode(view_mode: &str) -> Result<(), ValidationError> {
  match view_mode {
    "chatbox" | "article" => Ok(()),
    _ => Err(ValidationError::new("invalid_view_mode")),
  }
}

#[allow(dead_code)]
pub fn validate_uuid_v4(uuid: &str) -> Result<(), ValidationError> {
  match uuid.parse::<Uuid>() {
    Ok(res) => {
      if res.get_version_num() == 4 {
        Ok(())
      } else {
        Err(ValidationError::new("invalid_uuid"))
      }
    }
    Err(_) => Err(ValidationError::new("invalid_uuid")),
  }
}
