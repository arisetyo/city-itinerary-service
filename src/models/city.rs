use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct City {
    pub id: i32,
    pub city_name: String,
}
