use salvo::prelude::*;
use serde_json::json;

#[handler]
pub async fn create_itinerary(req: &mut Request, res: &mut Response) {
    let body: serde_json::Value = req.parse_json().await.unwrap();
    let city_name = body["city_name"].as_str().unwrap();

    // Placeholder: Call OpenAI and save to DB
    let itinerary = json!({
        "city_name": city_name,
        "itinerary": {
            "day_1": "Visit landmark A",
            "day_2": "Visit landmark B"
        }
    });

    res.render(Json(itinerary));
}

#[handler]
pub async fn get_itineraries(_req: &mut Request, res: &mut Response) {
    // Placeholder: Fetch itineraries from DB
    let itineraries = vec![
        json!({"city_name": "Paris", "itinerary": {"day_1": "Eiffel Tower", "day_2": "Louvre"}}),
        json!({"city_name": "Tokyo", "itinerary": {"day_1": "Shibuya Crossing", "day_2": "Meiji Shrine"}}),
    ];

    res.render(Json(itineraries));
}
