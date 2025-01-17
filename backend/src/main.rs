mod db;
mod errors;
mod handlers;
mod middle;
use axum::extract::ws::WebSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use db::new_postgres_conn;
use db::DynUserRepo;
use middle::auth_layer;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
#[derive(Clone)]
pub struct StaticState {
    pub wasm: Vec<u8>,
    pub login_page: String,
    pub loadjs: String,
    pub test_clients: Arc<Mutex<HashMap<String, ClientStruct>>>,
    pub send_page: String,
    pub css: String,
}
#[derive(Debug)]
pub struct ClientStruct{
    pub time: u128,
    pub socket: WebSocket
}

#[tokio::main]
async fn main() {
    // could be improved by just using leptos_axum and its provided functions
    let test_clients_map: HashMap<String, ClientStruct> = HashMap::new();
    let mutex_test_clients = Mutex::new(test_clients_map);
    let test_clients = Arc::new(mutex_test_clients);
    let wasm = std::fs::read("target/site/pkg/ws-explo.wasm").unwrap();
    let login_page = std::fs::read_to_string("views/login.html").unwrap();
    let send_page = std::fs::read_to_string("views/send.html").unwrap();
    let loadjs = std::fs::read_to_string("target/site/pkg/ws-explo.js").unwrap();
    let css = std::fs::read_to_string("target/site/pkg/ws-explo.css").unwrap();
    let state = StaticState {
        css,
        loadjs,
        wasm,
        login_page,
        test_clients,
        send_page,
    };
    let new_conn: db::PgConn = match new_postgres_conn(state).await {
        Ok(pool) => pool,
        Err(e) => panic!("Error: {}", e),
    };
    let appstate = Arc::new(new_conn) as DynUserRepo;
    let router = Router::new()
        .route("/publish", post(handlers::publish::handle_publish))
        .route(
            "/client_active",
            get(handlers::client_active::handle_client_active),
        )
        .route_layer(middleware::from_fn_with_state(appstate.clone(), auth_layer))
        .route("/main.wasm", get(handlers::wasm::handle_wasm))
        .route("/login_post", post(handlers::login::handle_login_post))
        .route("/register_post", post(handlers::register::handle_register_post))
        .route("/stylesheet.css", get(handlers::handle_ccs::handle_css))
        .route("/load.js", get(handlers::loadjs::handle_load_js))
        .route("/ws", get(handlers::ws::handle_ws))
        .route("/check_auth",post(handlers::check_auth::handle_check_auth))
        .fallback(handlers::fallback::fallback)
        .with_state(appstate.clone());

    let state_cloned = appstate.clone();
    tokio::spawn(async move {
        loop{
            tokio::time::sleep(Duration::from_secs(60)).await;
            let time_curr =  match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(t) => t,
                Err(_) => return
            };
            let state = state_cloned.state();
            let mut  clients = state.test_clients.lock().await;
            let mut clients_to_remove = Vec::new();
            for (k,v) in clients.iter(){
                if v.time + 300000 < time_curr.as_millis(){
                    clients_to_remove.push(k.clone());
                }
            }
            for client in clients_to_remove{
                clients.remove(&client);
            }
            println!("Cleaned map of clients active for longer than 5 mins");
        }

    });
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
