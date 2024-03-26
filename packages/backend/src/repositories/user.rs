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
        select *
        from users
        order by username
    "#,
  )
  .fetch_all(&*db)
  .await?;

  Ok(users)
}

pub async fn get_user(db: Extension<PgPool>, username: String) -> Result<UserModel, Error> {
  debug!("Get user...");
  let user = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
        select *
        from users
        where username = $1
        order by username
    "#,
    username
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
      INSERT INTO users (username, password, google_oauth, apple_oauth, discord_oauth, user_prompt_persona)
      VALUES ($1, $2, $3, $4, $5, $6)
      RETURNING *
    "#,
    user.username,
    user.password,
    user.googleOauth,
    user.appleOauth,
    user.discord_oauth,
    user.userPromptPersona
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
        password = COALESCE($1, password),
        google_oauth = COALESCE($2, google_oauth),
        apple_oauth = COALESCE($3, apple_oauth),
        discord_oauth = COALESCE($4, discord_oauth),
        user_prompt_persona = COALESCE($5, user_prompt_persona)
      WHERE username = $6
      RETURNING *
    "#,
    user.password,
    user.googleOauth,
    user.appleOauth,
    user.discordOauth,
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
