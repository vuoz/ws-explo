use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::db::DynUserRepo;

pub async fn handle_wasm(State(state): State<DynUserRepo>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", "application/wasm".parse().unwrap());
    (StatusCode::OK, headers, state.state().wasm).into_response()
}
