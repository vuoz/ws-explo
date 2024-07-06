use crate::errors::DbError;
use axum::extract::Json;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde::*;

use crate::db::DynUserRepo;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub key: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthedUser {
    pub token: String,
    pub key: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: u64,
}

pub async fn handle_login_post(
    State(state): State<DynUserRepo>,
    json_body: Json<User>,
) -> impl IntoResponse {
    let new_user = User {
        name: json_body.name.clone(),
        pass: json_body.pass.clone(),
        key: "".to_string(),
    };
    let new_user_for_fetch = new_user.clone();
    let user = match state.get_user(new_user_for_fetch).await {
        Ok(user) => user,
        Err(_) => return StatusCode::FORBIDDEN.into_response(),
    };
    //Password hashing is still going to be implemented
    if user.pass != json_body.pass {
        return StatusCode::FORBIDDEN.into_response();
    }
    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            exp: jsonwebtoken::get_current_timestamp(),
        },
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    ) {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(token) => token,
    };
    let key = user.key.clone();
    let new_authed_user = AuthedUser {
        token: token.clone(),
        key: key.clone(),
    };
    match state.add_user_auth(new_authed_user).await {
        Ok(()) => (),
        Err(e) => match e {
            DbError::NoResult => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            DbError::Error(e2) => match e2 {
                sqlx::Error::RowNotFound => (),
                _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        },
    };
    (
        StatusCode::OK,
        Json(AuthedUser {
            key: key.clone(),
            token,
        }),
    )
        .into_response()
}
