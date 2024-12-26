use sqlx::{PgPool, Pool, Postgres};
use crate::models::city::City;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await
}

pub async fn fetch_all_cities(pool: &Pool<Postgres>) -> Result<Vec<City>, sqlx::Error> {
    let rows = sqlx::query_as!(City, "SELECT id, name FROM cities")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
