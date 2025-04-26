use sqlx::{SqlitePool, Error};

pub type DbPool = SqlitePool;

pub async fn get_db_pool() -> Result<DbPool, Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    SqlitePool::connect(&db_url).await
}
