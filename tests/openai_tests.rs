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

use serde_json::json;
use city_itinerary_service::handlers::openai::fetch_openai_response;

/// This test function mocks the OpenAI API response and tests the `fetch_openai_response` function.
#[tokio::test]
async fn test_fetch_openai_response() {
    println!("-----> Starting test_fetch_openai_response");

    let city_name = "New York";
    let response = fetch_openai_response(city_name).await.unwrap();

    // Print the response in a readable format
    println!("-----> Response: {}", serde_json::to_string_pretty(&response).unwrap());
}
