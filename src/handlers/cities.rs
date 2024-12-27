use salvo::prelude::*;
use serde_json::json;
use crate::db;
use crate::models::city::City;

pub async fn fetch_all_cities(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<City>, sqlx::Error> {
    let rows = sqlx::query_as!(
        City,
        r#"
        SELECT id, city_name, created_at
        FROM cities
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn save_city(pool: &sqlx::Pool<sqlx::Postgres>, city: &City) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        City,
        r#"
        INSERT INTO cities (city_name, created_at)
        VALUES ($1, $2)
        RETURNING id, city_name, created_at
        "#,
        city.city_name,
        city.created_at
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}


#[handler]
pub async fn get_cities(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let cities = fetch_all_cities(&pool).await.unwrap();
    res.render(Json(cities));
}

#[handler]
pub async fn create_city(req: &mut Request, res: &mut Response) {
    let body: serde_json::Value = req.parse_json().await.unwrap();
    let city_name = body["city_name"].as_str().unwrap();

    let city_data = City {
        id: 0,
        city_name: city_name.to_string(),
        created_at: chrono::Local::now().naive_local(),
    };

    let pool = db::init_db().await.unwrap();
    match save_city(&pool, &city_data).await {
        Ok(saved_city) => {
            res.render(Json(saved_city));
        }
        Err(e) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"error": e.to_string()})));
        }
    }
}