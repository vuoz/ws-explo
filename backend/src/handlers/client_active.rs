use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::handlers::login::User;
use axum::{Extension, Json};
use axum::extract::State;

use crate::db::DynUserRepo;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClientActive{
    active:bool
}

pub async fn handle_client_active(
    Extension(user): Extension<User>,
    State(state):State<DynUserRepo>) -> impl IntoResponse{
    if state.state().test_clients.lock().await.contains_key(&user.key){
         Json(ClientActive{active:true}).into_response()
    }else{
         Json(ClientActive{active:false}).into_response()
    }
}
