use crate::{
  error::Error,
  middlewares::check_auth,
  repositories::user,
  utils::json::json_wrapper,
  validation::user::{CreateUserSchema, UpdateUserSchema},
};
use axum::{extract::Path, middleware, response::IntoResponse, routing::*, Extension, Json};
use sqlx::PgPool;

// TODO: Should add password hashing, validation, and other security measures
async fn create_user(
  db: Extension<PgPool>,
  Json(payload): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, Error> {
  let user = user::create_user(db, payload).await?;
  Ok(json_wrapper(user))
}

async fn get_users(db: Extension<PgPool>) -> Result<impl IntoResponse, Error> {
  let users = user::get_users(db).await?;
  Ok(json_wrapper(users))
}

async fn get_user(
  db: Extension<PgPool>,
  Path(username): Path<String>,
) -> Result<impl IntoResponse, Error> {
  let user = user::get_user(db, username).await?;
  Ok(json_wrapper(user))
}

async fn update_user(
  db: Extension<PgPool>,
  Path(username): Path<String>,
  Json(payload): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, Error> {
  let user = user::update_user(db, username, payload).await?;
  Ok(json_wrapper(user))
}

async fn delete_user(
  db: Extension<PgPool>,
  Path(username): Path<String>,
) -> Result<impl IntoResponse, Error> {
  let user = user::remove_user(db, username).await?;
  Ok(json_wrapper(user))
}

pub fn create_router() -> Router {
  Router::new()
    .route("/users", post(create_user).get(get_users))
    .route(
      "/users/:username",
      get(get_user).patch(update_user).delete(delete_user),
    )
    .route_layer(middleware::from_fn(check_auth::get_middleware_fn))
}
