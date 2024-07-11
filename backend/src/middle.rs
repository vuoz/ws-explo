use crate::db::DynUserRepo;
use crate::handlers::login::User;
use axum::extract::State;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::Deserialize;
use serde::Serialize;
use std::result::Result;


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct UserWithSession{
    pub user:User,
    pub session:String,
}
pub async fn auth_layer<B>(
    State(state): State<DynUserRepo>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let header_opt = req.headers().get("x-auth");
    let header = match header_opt.or(None) {
        Some(header) => header,
        None => return Err(StatusCode::FORBIDDEN),
    };
    let header_str_res = header.to_str();
    let header_str = match header_str_res.ok() {
        Some(header_str) => header_str,
        None => return Err(StatusCode::FORBIDDEN),
    };
    let user = match state.auth_user(header_str.to_string()).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::FORBIDDEN)
    };

    let user_with_session = UserWithSession{
        user,
        session:header_str.to_string()
    };
    req.extensions_mut().insert(user_with_session);
    let response = next.run(req).await;
    Ok(response)
}
