use axum::extract::State;
use axum::response::{Html, IntoResponse};

use crate::db::DynUserRepo;

pub async fn fallback(State(state): State<DynUserRepo>) -> impl IntoResponse {
    Html(state.state().login_page)
}
