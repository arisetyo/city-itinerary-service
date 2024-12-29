use salvo::prelude::*;
use serde_json::json;
use chrono::Utc;
use tokio::time::{sleep, Duration};
use crate::db;
use crate::models::itinerary::Itinerary;
use crate::handlers::openai::fetch_openai_response;


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

pub async fn check_itinerary_exists(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32) -> Result<bool, sqlx::Error> {
    let current_date = Utc::now().naive_utc().date();
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*)
        FROM itineraries
        WHERE user_id = $1 AND DATE(created_at) = $2
        "#,
        user_id,
        current_date
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count.unwrap_or(0) > 0)
}


#[handler]
pub async fn get_city_itinerary(req: &mut Request, res: &mut Response) {
    let city_name = req.query::<String>("city_name").unwrap();
    let user_id = req.query::<i32>("user_id").unwrap();

    let pool = db::init_db().await.unwrap();

    if check_itinerary_exists(&pool, user_id).await.unwrap() {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Json(json!({"error": "Itinerary already exists for today"})));
        return;
    }

    // Rate limiter: wait for 1 second before making the request
    sleep(Duration::from_secs(1)).await;

    let city_name_clone = city_name.clone();
    let fetch_future = tokio::spawn(async move {
        fetch_openai_response(&city_name_clone).await
    });

    match fetch_future.await.unwrap() {
        Ok(itinerary) => {
            let new_itinerary = Itinerary {
                id: 0,
                user_id,
                city_name: city_name.clone(),
                itinerary: itinerary.clone(),
                created_at: Utc::now().naive_utc(),
            };

            sqlx::query!(
                r#"
                INSERT INTO itineraries (user_id, city_name, itinerary, created_at)
                VALUES ($1, $2, $3, $4)
                "#,
                new_itinerary.user_id,
                new_itinerary.city_name,
                new_itinerary.itinerary,
                new_itinerary.created_at
            )
            .execute(&pool)
            .await
            .unwrap();

            res.render(Json(new_itinerary));
        }
        Err(e) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"error": e.to_string()})));
        }
    }
}


#[handler]
pub async fn get_itineraries(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let itineraries = fetch_all_itineraries(&pool).await.unwrap();
    res.render(Json(itineraries));
}
