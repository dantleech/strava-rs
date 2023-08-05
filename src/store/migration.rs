

use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) {

    sqlx::migrate!("./migrations").run(pool).await?;


}
