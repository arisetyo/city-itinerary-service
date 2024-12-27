use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Itinerary {
    pub id: i32,
    pub user_id: i32,
    pub city_name: String,
    pub itinerary: serde_json::Value,
    pub created_at: NaiveDateTime,
}
