use axum::{
    routing::post,
    Router,
};
use crate::person::person_controller::create_person;

pub fn person_routes() -> Router {
    Router::new()
        .route("/", post(create_person))
}
