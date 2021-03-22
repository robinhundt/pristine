use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::env;
use std::str::FromStr;

pub async fn get_db_pool() -> SqlitePool {
    let options =
        SqliteConnectOptions::from_str(&env::var("DATABASE_URL").expect("DATABASE_ULR not set"))
            .unwrap()
            .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}
