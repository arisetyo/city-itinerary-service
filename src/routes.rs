use salvo::prelude::*;
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .post(handlers::create_itinerary)
        .get(handlers::get_itineraries)
        .get(handlers::cities::get_cities)
}
