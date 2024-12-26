use salvo::prelude::*;
use crate::db;
use crate::models::city::City;

#[handler]
pub async fn get_cities(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let cities = db::fetch_all_cities(&pool).await.unwrap();
    res.render(Json(cities));
}
