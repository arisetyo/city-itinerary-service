use salvo::prelude::*;
use serde_json::json;
use sqlx::PgPool;
use crate::models::itinerary::Itinerary;

#[handler]
pub async fn create_itinerary(req: &mut Request, res: &mut Response) {
    let body: serde_json::Value = req.parse_json().await.unwrap();
    let city_name = body["city_name"].as_str().unwrap();
    let itinerary = body["itinerary"].clone();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let itinerary_data = Itinerary {
        city_name: city_name.to_string(),
        itinerary: itinerary,
    };

    sqlx::query!(
        "INSERT INTO itineraries (user_id, city_name, itinerary) VALUES ($1, $2, $3)",
        1, // Placeholder for user_id
        itinerary_data.city_name,
        itinerary_data.itinerary
    )
    .execute(&pool)
    .await
    .unwrap();

    res.render(Json(itinerary_data));
}

#[handler]
pub async fn get_itineraries(_req: &mut Request, res: &mut Response) {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let rows = sqlx::query!(
        "SELECT city_name, itinerary FROM itineraries"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let itineraries: Vec<Itinerary> = rows.into_iter().map(|row| Itinerary {
        city_name: row.city_name,
        itinerary: row.itinerary,
    }).collect();

    res.render(Json(itineraries));
}
