use crate::errors::DbError;
use crate::handlers::login::User;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::db::DynUserRepo;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegUser {
    name: String,
    pass: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegReturn {
    key: String,
    auth: String,
}
pub async fn handle_register_post(
    State(state): State<DynUserRepo>,
    json_body: Json<RegUser>,
) -> impl IntoResponse {
    let new_key = uuid::Uuid::new_v4().to_string();
    let new_user = User {
        name: json_body.name.clone(),
        pass: json_body.pass.clone(),
        key: new_key.clone(),
        user_id: "".to_string(),
    };
    match state.add_user(new_user).await {
        Ok(()) => (),
        Err(e) => match e {
            DbError::NoResult => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            DbError::Error(sqlx::Error::RowNotFound) => () ,
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };
    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &crate::handlers::login::Claims {
            exp: jsonwebtoken::get_current_timestamp(),
        },
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    ) {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(token) => token,
    };
    // need to add auth
    let new_return = crate::handlers::login::AuthedUser {
        key: new_key,
        token,
    };

    match state.add_user_auth(new_return.clone()).await {
        Ok(()) => (),
        Err(e) => {
            println!("{}",e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
    };
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    (StatusCode::OK, headers, Json(new_return)).into_response()
}
