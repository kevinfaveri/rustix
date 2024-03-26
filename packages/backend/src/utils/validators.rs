use uuid::Uuid;
use validator::ValidationError;

pub fn validate_user_prompt_persona(arg: &Option<String>) -> Result<(), ValidationError> {
  if let Some(persona) = arg {
    match persona.as_str() {
      "alfred" | "trippy" => Ok(()),
      _ => Err(ValidationError::new("invalid_user_prompt_persona")),
    }
  } else {
    Ok(())
  }
}

pub fn validate_view_mode(arg: &Option<String>) -> Result<(), ValidationError> {
  if let Some(view_mode) = arg {
    match view_mode.as_str() {
      "chatbox" | "article" => Ok(()),
      _ => Err(ValidationError::new("invalid_view_mode")),
    }
  } else {
    Ok(())
  }
}

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
