use salvo::prelude::*;
use crate::db;
use crate::models::city::City;

pub async fn fetch_all_cities(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<City>, sqlx::Error> {
    let rows = sqlx::query_as!(City, "SELECT id, city_name FROM cities")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

#[handler]
pub async fn get_cities(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let cities = fetch_all_cities(&pool).await.unwrap();
    res.render(Json(cities));
}
