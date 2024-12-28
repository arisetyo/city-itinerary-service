/// To run the test in this file, use the following command:
///
/// ```sh
/// cargo test handlers::tests::openai_tests
/// ```
///
/// For a specific function, you can specify the function name:
/// ```sh
/// cargo test handlers::tests::openai_tests::test_fetch_openai_response
/// ```
///
/// This command will execute the `test_fetch_openai_response` function, which tests the `fetch_openai_response`
/// function from the `handlers::openai` module. The test uses the `mockito` crate to mock the OpenAI API response.

use mockito::mock;
use serde_json::json;
use crate::handlers::openai::fetch_openai_response;

/// This test function mocks the OpenAI API response and tests the `fetch_openai_response` function.
#[tokio::test]
async fn test_fetch_openai_response() {
    println!("Starting test_fetch_openai_response");

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

    // Print the response in a readable format
    println!("Response: {}", serde_json::to_string_pretty(&response).unwrap());

    let expected_response = json!({
        "day1": "Visit Central Park",
        "day2": "See the Statue of Liberty"
    });

    println!("Expected response: {}", serde_json::to_string_pretty(&expected_response).unwrap());

    assert_eq!(response, expected_response);
}
