use reqwest::Client;
use serde_json::Value;
use std::env;

pub async fn fetch_openai_response(city_name: &str) -> Result<Value, reqwest::Error> {
    // This is needed for test because the test is not running in the same context as the main application
    dotenv::dotenv().ok(); // Load .env file

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new();
    let request_body = serde_json::json!({
        "model": "text-davinci-003",
        "prompt": format!("Provide a day-to-day activity for a tourist visiting {}", city_name),
        "max_tokens": 100,
    });

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    let itinerary = response_json["choices"][0]["text"].as_str().unwrap_or("").trim();

    let mut itinerary_json = serde_json::Map::new();
    for (i, activity) in itinerary.split('\n').enumerate() {
        itinerary_json.insert(format!("day{}", i + 1), Value::String(activity.to_string()));
    }

    Ok(Value::Object(itinerary_json))
}
