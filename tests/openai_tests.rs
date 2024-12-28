use mockito::mock;
use serde_json::json;
use crate::handlers::openai::fetch_openai_response;

#[tokio::test]
async fn test_fetch_openai_response() {
    let _m = mock("POST", "/v1/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "choices": [
                    {
                        "text": "Visit Central Park\nSee the Statue of Liberty"
                    }
                ]
            })
            .to_string(),
        )
        .create();

    let city_name = "New York";
    let response = fetch_openai_response(city_name).await.unwrap();

    let expected_response = json!({
        "day1": "Visit Central Park",
        "day2": "See the Statue of Liberty"
    });

    assert_eq!(response, expected_response);
}
