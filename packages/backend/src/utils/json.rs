use axum::Json;
use serde::Serialize;
use serde_json::json;

pub fn json_wrapper_paginated<T: Serialize>(
  data: T,
  offset: Option<usize>,
  limit: Option<usize>,
) -> Json<serde_json::Value> {
  let json_data = serde_json::to_value(data).unwrap();
  let meta_value = if let (Some(offset), Some(limit)) = (offset, limit) {
    Some(json!({
      "offset": offset,
      "limit": limit
    }))
  } else {
    None
  };

  let json_value = if let Some(meta_value) = meta_value {
    json!({
        "result": json_data,
        "meta": meta_value
    })
  } else {
    json!({
        "result": json_data
    })
  };

  Json(json_value)
}

pub fn json_wrapper<T: Serialize>(data: T) -> Json<serde_json::Value> {
  json_wrapper_paginated(data, None, None)
}
