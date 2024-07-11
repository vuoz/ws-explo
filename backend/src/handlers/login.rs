use axum::extract::Json;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde::*;

use crate::db::DynUserRepo;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub key: String,
    pub user_id :String,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLoginReq{
    pub name :String,
    pub pass : String,
}


pub async fn handle_login_post(
    State(state): State<DynUserRepo>,
    json_body: Json<UserLoginReq>,
) -> impl IntoResponse {
    let new_user = User {
        name: json_body.name.clone(),
        pass: json_body.pass.clone(),
        key: "".to_string(),
        user_id:"".to_string()
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
        Err(_) =>return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(token) => token,
    };
    let key_cloned = user.key.clone();
    let token_cloned = token.clone();
    match state.add_user_auth(user,token).await {
        Ok(()) => (),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    (
        StatusCode::OK,
        Json(AuthedUser {
            key: key_cloned,
            token:token_cloned,
        }),
    )
        .into_response()
}
