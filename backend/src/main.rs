mod db;
mod errors;
mod handlers;
mod middle;
use axum::extract::ws::WebSocket;
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
use tokio::sync::Mutex;
#[derive(Clone)]
pub struct StaticState {
    pub wasm: Vec<u8>,
    pub login_page: String,
    pub loadjs: String,
    pub test_clients: Arc<Mutex<HashMap<String, WebSocket>>>,
    pub send_page: String,
    pub css:String
}

#[tokio::main]
async fn main() {
    // could be improved by just using leptos_axum and its provided functions
    let wasm = std::fs::read("target/site/pkg/ws-explo.wasm").unwrap();
    let login_page = std::fs::read_to_string("views/login.html").unwrap();
    let send_page = std::fs::read_to_string("views/send.html").unwrap();
    let loadjs = std::fs::read_to_string("target/site/pkg/ws-explo.js").unwrap();
    let css = std::fs::read_to_string("target/site/pkg/ws-explo.css").unwrap();

    let test_clients_map: HashMap<String, WebSocket> = HashMap::new();
    let mutex_test_clients = Mutex::new(test_clients_map);
    let test_clients = Arc::new(mutex_test_clients); 

    let state = StaticState {
        css,
        loadjs,
        wasm,
        login_page,
        test_clients,
        send_page,
    };
    let new_conn: db::PgConn = new_postgres_conn(state).await;
    let appstate = Arc::new(new_conn) as DynUserRepo;
    let router = Router::new()
        .route("/publish", post(handlers::publish::handle_publish))
        .route("/client_active",get(handlers::client_active::handle_client_active))
        .route_layer(middleware::from_fn_with_state(appstate.clone(), auth_layer))
        .route("/main.wasm", get(handlers::wasm::handle_wasm))
        .route("/login", post(handlers::login::handle_login_post))
        .route("/register", post(handlers::register::handle_register_post))
        .route("/stylesheet.css", get(handlers::handle_ccs::handle_css))
        .route("/load.js", get(handlers::loadjs::handle_load_js))
        .route("/ws", get(handlers::ws::handle_ws))
        .fallback(handlers::fallback::fallback)
        .with_state(appstate);
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
