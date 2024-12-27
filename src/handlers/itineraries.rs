use salvo::prelude::*;
use crate::db;
use crate::models::itinerary::Itinerary;

pub async fn fetch_all_itineraries(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Itinerary>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Itinerary,
        r#"
        SELECT id, user_id, city_name, itinerary, created_at
        FROM itineraries
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[handler]
pub async fn get_itineraries(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let itineraries = fetch_all_itineraries(&pool).await.unwrap();
    res.render(Json(itineraries));
}