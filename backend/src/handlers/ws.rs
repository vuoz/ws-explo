use std::time::{ SystemTime, UNIX_EPOCH};

use crate::ClientStruct;
use crate::db::DynUserRepo;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;

pub async fn handle_ws(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<DynUserRepo>,
) -> impl IntoResponse {
    let header = match headers.get("x-key") {
        Some(header) => header,
        None => return StatusCode::FORBIDDEN.into_response(),
    };
    let header_str = match header.to_str() {
        Ok(header) => header,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let header_string = header_str.to_string();

    ws.on_upgrade(move |socket| handle_socket(socket, header_string, state))
}

async fn handle_socket(mut socket: WebSocket, user: String, state: DynUserRepo) {
    match socket.send(Message::Ping(vec![1, 2, 3])).await {
        Ok(e) => e,
        Err(_) => return,
    };
    let appstate = state.state();
    let mut clients = appstate.test_clients.lock().await;
    let time =  match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => t,
        Err(_) => return
    };
    let c = ClientStruct{
            time:time.as_millis(),
            socket,
    };
    clients.insert(user, c);
}
