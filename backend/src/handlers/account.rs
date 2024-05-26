use crate::db::DynUserRepo;
use axum::http::HeaderMap;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn handle_account_get(State(state): State<DynUserRepo>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", "text/html".parse().unwrap());
    (StatusCode::OK, headers, state.state().login_page).into_response();
}
