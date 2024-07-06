use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::db::DynUserRepo;

pub async fn handle_css(State(state): State<DynUserRepo>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", "text/css".parse().unwrap());
    (StatusCode::OK, headers, state.state().css).into_response()
}
