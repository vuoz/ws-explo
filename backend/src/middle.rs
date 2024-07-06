use crate::db::DynUserRepo;
use axum::extract::State;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::result::Result;
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
        Err(_) => return Err(StatusCode::FORBIDDEN),
    };

    req.extensions_mut().insert(user);
    let response = next.run(req).await;
    Ok(response)
}
