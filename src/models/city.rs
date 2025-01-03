use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct City {
    pub id: i32,
    pub city_name: String,
    pub created_at: NaiveDateTime,
}
