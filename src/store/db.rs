use sqlx::SqlitePool;

pub async fn get_pool(path: String) -> SqlitePool {
    
    SqlitePool::connect(format!("sqlite://{}?mode=rwc", path).as_str()).await.expect("Could not connect to DB")
}
