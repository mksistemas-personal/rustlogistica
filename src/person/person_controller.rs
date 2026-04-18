use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use crate::person::person_data::PersonRequest;

pub async fn create_person(Json(payload): Json<PersonRequest>) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => (StatusCode::CREATED, Json(payload)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
