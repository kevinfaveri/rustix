use crate::{
  error::Error,
  models::user::UserModel,
  validation::user::{CreateUserSchema, UpdateUserSchema},
};
use axum::Extension;
use sqlx::PgPool;
use tracing::{debug, error};

pub async fn get_users(db: Extension<PgPool>) -> Result<Vec<UserModel>, Error> {
  debug!("Getting users...");
  let result = sqlx::query_as!(
    UserModel,
    // language=PostgreSQL
    r#"
        select id, username, user_name, user_avatar, user_prompt_persona, created_at, updated_at
        from users
        order by username
    "#,
  )
  .fetch_all(&*db)
  .await;

  match result {
    Ok(users) => Ok(users),
    Err(err) => {
      error!("Failed to get users: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn get_user(db: Extension<PgPool>, username_or_id: String) -> Result<UserModel, Error> {
  debug!("Get user...");
  let result = sqlx::query_as!(
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
  .await;

  match result {
    Ok(user) => Ok(user),
    Err(err) => {
      error!("Failed to get user: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn create_user(
  db: Extension<PgPool>,
  user: CreateUserSchema,
) -> Result<UserModel, Error> {
  debug!("Creating user...");
  let result = sqlx::query_as!(
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
  .await;

  match result {
    Ok(user) => Ok(user),
    Err(err) => {
      error!("Failed to create user: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn update_user(
  db: Extension<PgPool>,
  username: String,
  user: UpdateUserSchema,
) -> Result<UserModel, Error> {
  let result = sqlx::query_as!(
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
  .await;

  match result {
    Ok(user) => Ok(user),
    Err(err) => {
      error!("Failed to update user: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}

pub async fn remove_user(db: Extension<PgPool>, username: String) -> Result<(), Error> {
  let result = sqlx::query!(
    r#"
      DELETE FROM users
      WHERE username = $1
    "#,
    username
  )
  .execute(&*db)
  .await;

  match result {
    Ok(_) => Ok(()),
    Err(err) => {
      error!("Failed to delete user: {:?}", err);
      Err(Error::Sqlx(err))
    }
  }
}
