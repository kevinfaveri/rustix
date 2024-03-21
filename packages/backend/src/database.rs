use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::settings::SETTINGS;

pub async fn start_db() -> anyhow::Result<PgPool> {
  info!("Starting database...");
  let db = PgPoolOptions::new()
    .max_connections(20)
    .connect(&SETTINGS.database_uri)
    .await
    .context("failed to connect to DATABASE_URL")?;
  info!("Connected to database, running migrations...");

  sqlx::migrate!().run(&db).await?;

  info!("Migrations run successfully, starting app server...");

  Ok(db)
}
