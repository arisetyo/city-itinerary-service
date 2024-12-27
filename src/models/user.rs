use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub google_id: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
