use crate::{
  error::Error,
  models::user::UserModel,
  validation::user::{CreateUserSchema, UpdateUserSchema},
};
use axum::Extension;
use sqlx::PgPool;
use tracing::debug;

pub async fn get_users(db: Extension<PgPool>) -> Result<Vec<UserModel>, Error> {
  debug!("Getting users...");
  let users = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
        select id, username, user_name, user_avatar, user_prompt_persona, created_at, updated_at
        from users
        order by username
    "#,
  )
  .fetch_all(&*db)
  .await?;

  Ok(users)
}

pub async fn get_user(db: Extension<PgPool>, username_or_id: String) -> Result<UserModel, Error> {
  debug!("Get user...");
  let user = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
        select id, username, user_name, user_avatar, user_prompt_persona, created_at, updated_at
        from users
        where username = $1 or id = $1::uuid
    "#,
    username_or_id
  )
  .fetch_one(&*db)
  .await?;

  Ok(user)
}

pub async fn create_user(
  db: Extension<PgPool>,
  user: CreateUserSchema,
) -> Result<UserModel, Error> {
  let user = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
      INSERT INTO users (username, user_name, user_avatar, user_prompt_persona)
      VALUES ($1, $2, $3, $4)
      RETURNING *
    "#,
    user.username,
    user.user_name,
    user.user_avatar,
    user.user_prompt_persona
  )
  .fetch_one(&*db)
  .await?;

  Ok(user)
}

pub async fn update_user(
  db: Extension<PgPool>,
  username: String,
  user: UpdateUserSchema,
) -> Result<UserModel, Error> {
  let user = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
      UPDATE users
      SET 
        user_name = COALESCE($1, user_name),
        user_avatar = COALESCE($2, user_avatar),
        user_prompt_persona = COALESCE($3, user_prompt_persona)
      WHERE username = $4
      RETURNING *
    "#,
    user.user_name,
    user.user_avatar,
    user.userPromptPersona,
    username
  )
  .fetch_one(&*db)
  .await?;

  Ok(user)
}

pub async fn remove_user(db: Extension<PgPool>, username: String) -> Result<(), Error> {
  sqlx::query!(
    r#"
      DELETE FROM users
      WHERE username = $1
    "#,
    username
  )
  .execute(&*db)
  .await?;

  Ok(())
}
