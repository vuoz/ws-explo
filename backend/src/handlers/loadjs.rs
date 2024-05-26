use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};

use crate::db::DynUserRepo;
use axum::response::IntoResponse;

pub async fn handle_load_js(State(state): State<DynUserRepo>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", "text/javascript".parse().unwrap());
    (StatusCode::OK, headers, state.state().loadjs).into_response()
}
