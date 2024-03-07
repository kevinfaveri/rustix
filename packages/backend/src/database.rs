use crate::settings::SETTINGS;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn connection() -> &'static PgPool {
    POOL.get_or_init(|| async {
        let connect_options = PgConnectOptions::new()
            .username(SETTINGS.database.user.as_str())
            .password(SETTINGS.database.password.as_str())
            .host(SETTINGS.database.host.as_str())
            .port(SETTINGS.database.port)
            .database(SETTINGS.database.name.as_str());

        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await
            .expect("Failed to connect to PostgreSQL database")
    })
    .await
}
