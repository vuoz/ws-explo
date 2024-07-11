use crate::middle::UserWithSession;
use crate:: db::DynUserRepo ;
use axum::http::StatusCode;
use axum::{
    extract::{ws::Message, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebMsg {
    msg: String,
    close: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResp {
    error: String,
}

pub async fn handle_publish(
    Extension(user): Extension<UserWithSession>,
    State(state): State<DynUserRepo>,
    Json(msg): Json<WebMsg>,
) -> impl IntoResponse {
    let appstate = state.state();
    let mut clients = appstate.test_clients.lock().await;
    let client = match clients.get_mut(&user.user.key) {
        Some(client) => client,
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(ErrorResp {
                    error: String::from("Client is not active!"),
                }),
            )
                .into_response()
        }
    };
    let msg_str = match serde_json::to_string(&msg) {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResp {
                    error: String::from("Error with your message!"),
                }),
            )
                .into_response()
        }
        Ok(msg) => msg,
    };
    match client.socket.send(Message::Text(msg_str)).await {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResp {
                    error: String::from("Error sending to client!"),
                }),
            )
                .into_response()
        }
        Ok(ad) => ad,
    };
    // Option to cancel client
    if msg.close {
        clients.remove(&user.user.key);
    }

    StatusCode::OK.into_response()
}
