use crate::{
  error::Error,
  models::user::UserModel,
  repositories::user,
  validation::user::{CreateUserSchema, UpdateUserSchema},
};
use axum::{extract::Path, middleware, routing::*, Extension, Json};
use sqlx::PgPool;

use super::middlewares::check_auth;

// TODO: Should add password hashing, validation, and other security measures
async fn create_user(
  db: Extension<PgPool>,
  Json(payload): Json<CreateUserSchema>,
) -> Result<Json<UserModel>, Error> {
  let user = user::create_user(db, payload).await?;
  Ok(Json(user))
}

async fn get_users(db: Extension<PgPool>) -> Result<Json<Vec<UserModel>>, Error> {
  let users = user::get_users(db).await?;
  Ok(Json(users))
}

async fn get_user(
  db: Extension<PgPool>,
  Path(username): Path<String>,
) -> Result<Json<UserModel>, Error> {
  let user = user::get_user(db, username).await?;
  Ok(Json(user))
}

async fn update_user(
  db: Extension<PgPool>,
  Path(username): Path<String>,
  Json(payload): Json<UpdateUserSchema>,
) -> Result<Json<UserModel>, Error> {
  let user = user::update_user(db, username, payload).await?;
  Ok(Json(user))
}

async fn delete_user(db: Extension<PgPool>, Path(username): Path<String>) -> Result<(), Error> {
  user::remove_user(db, username).await?;
  Ok(())
}

pub fn create_router() -> Router {
  Router::new()
    .route("/users", post(create_user).get(get_users))
    .route(
      "/users/:username",
      get(get_user).patch(update_user).delete(delete_user),
    )
    .route_layer(middleware::from_fn(check_auth))
}
