/// This function creates and returns a router with the following endpoints:
///
/// - `GET  /itineraries`   : Calls the `get_itineraries` handler to retrieve all itineraries.
/// - `POST /city`          : Calls the `create_city` handler to create a new city.
/// - `GET  /cities`        : Calls the `get_cities` handler to retrieve all cities.
/// - `GET  /users`         : Calls the `get_users` handler to retrieve all users.

use salvo::prelude::*;
use crate::handlers::cities;
use crate::handlers::itineraries;
use crate::handlers::users;

pub fn create_router() -> Router {
    Router::new()
    .push(Router::new().path("itineraries")
        .get(itineraries::get_itineraries))
    .push(Router::new().path("cities")
        .get(cities::get_cities))
    .push(Router::new().path("users")
        .get(users::get_users))
    .push(Router::new().path("city")
        .post(cities::create_city))
}
