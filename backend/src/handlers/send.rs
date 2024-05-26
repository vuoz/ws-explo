use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::db::DynUserRepo;

pub async fn handle_send(State(state): State<DynUserRepo>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", "text/html".parse().unwrap());
    (StatusCode::OK, headers, state.state().send_page).into_response()
}
