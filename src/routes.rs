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
    .push(Router::new().path("user/<id>")
        .get(users::get_user_by_id))
    .push(Router::new().path("city")
        .post(cities::create_city))
    .push(Router::new().path("city_itinerary")
        .get(itineraries::get_city_itinerary))
}
