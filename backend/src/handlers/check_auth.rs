use axum::http::StatusCode;
use axum::response::IntoResponse;


use axum::extract::Json;
use axum::extract::State;
use crate::db::DynUserRepo;

use super::login::AuthedUser;

pub async fn handle_check_auth(State(state):State<DynUserRepo>,Json(user):Json<AuthedUser>)-> impl IntoResponse{
    match state.auth_user(user.token).await{
        Ok(_) =>  StatusCode::OK.into_response(),
        Err(_) => StatusCode::FORBIDDEN.into_response()
    }
}
