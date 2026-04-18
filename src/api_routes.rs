use axum::{
    routing::get,
    Router,
};
use crate::person::person_routes::person_routes;

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Olá do Axum!" }))
        .nest("/api/person", person_routes())
}
