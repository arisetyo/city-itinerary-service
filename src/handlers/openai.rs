// use reqwest::Client;
use serde_json::Value;
use std::env;
use async_openai::{types::CreateCompletionRequestArgs, Client, config::OpenAIConfig};

pub async fn fetch_openai_response(city_name: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    // This is needed for test because the test is not running in the same context as the main application
    dotenv::dotenv().ok(); // Load .env file

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let org_id = env::var("OPENAI_ORGANIZATION").expect("OPENAI_ORGANIZATION must be set");

    let prompt = format!("Provide a day-to-day activity for a tourist visiting {} for 3 days. Return the answer in a JSON format.", city_name);

    // DEBUG
    println!("prompt: {}", prompt); // TODO: Remove this line

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(org_id);

    let client = Client::with_config(config);

    // single
    let request = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-instruct")
        .prompt(prompt)
        .max_tokens(240_u32)
        .build()?;

    let response = client.completions().create(request).await?;
    let itinerary_result = response.choices[0].text.trim();
    // DEBUG
    println!("itinerary_result: {}", itinerary_result); // TODO: Remove this line

    // Convert the CSV format string to a JSON object
    let mut itinerary_json = serde_json::Map::new();
    for (i, activity) in itinerary_result.split(',').enumerate() {
        let day = format!("day {}", i + 1);
        itinerary_json.insert(day, Value::String(activity.trim().to_string()));
    }

    Ok(Value::Object(itinerary_json))
}
