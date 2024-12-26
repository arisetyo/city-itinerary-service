/// This function creates and returns a router with the following endpoints:
///
/// - `POST /`: Calls the `create_itinerary` handler to create a new itinerary.
/// - `GET /`: Calls the `get_itineraries` handler to retrieve all itineraries.
/// - `GET /cities`: Calls the `get_cities` handler to retrieve all cities.

use salvo::prelude::*;
use crate::handlers::cities;
use crate::handlers::itineraries;
// use crate::handlers::itineraries::create_itinerary;
// use crate::handlers::itineraries::get_itineraries;

pub fn create_router() -> Router {
    Router::new()
    .push(Router::new().path("itineraries")
        .post(itineraries::create_itinerary)
        .get(itineraries::get_itineraries))
    .push(Router::new().path("cities")
        .get(cities::get_cities))
}
