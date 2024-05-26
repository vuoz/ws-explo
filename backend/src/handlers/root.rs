use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn handle_root() -> impl IntoResponse {
    (StatusCode::OK, "Hello World!".to_string()).into_response()
}
