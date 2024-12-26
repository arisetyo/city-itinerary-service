use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Itinerary {
    pub city_name: String,
    pub itinerary: serde_json::Value,
}
