use std::env::{self};
use std::collections::HashMap;

use axum::{
    routing::{get, put},
    http::{StatusCode, Uri},
    Json,
    Router,
    response::{IntoResponse},
    extract::{State}
};

use tracing::{info, Level};
use tracing_subscriber;

#[path="../store/mod.rs"]
pub mod store;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .with_target(false)
        .init();

    let http_port : u32 = env::var("HTTP_PORT")
        .unwrap_or(3030.to_string())
        .parse()
        .unwrap();

    // Prepare cache handler
    let kv_store = Box::new(store::kv::inmem::InMemory::new(100));

    // build our application with a route
    let app = Router::new()
        .route("/cache",put(put_cache_handler))
        .route("/caches/", get(get_cache))
        .fallback(not_found)
        .with_state(kv_store);

    let server_addr = format!("0.0.0.0:{}", http_port);

    info!("Server is up at address: {}", server_addr);

    // run our app with hyper, listening globally on port 3000
    // run it with hyper on localhost:3030
    axum::Server::bind(&server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn put_cache_handler(State(mut kv_store): State<Box<dyn store::kv::Storage>>) -> impl IntoResponse {
        info!("Put cache");
        _ = kv_store.put("a".to_string(), "a".to_string());
        StatusCode::OK
}

async fn get_cache() -> (StatusCode, Json<Vec<store::kv::entity::KV>>){
    info!("Put cache");

    let mut caches: Vec<store::kv::entity::KV> = Vec::new();

    let kv = store::kv::entity::KV {
        key: "a".to_string(),
        value: "a".to_string(),
    };

    caches.push(kv);

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(caches))
}

// async fn not_found(uri: Uri) -> (StatusCode, String) {
//     (StatusCode::NOT_FOUND, format!("No route for {}", uri))
// }

async fn not_found(uri: Uri) -> (StatusCode, Json<HashMap<&'static str, String>>) {
    let mut data = HashMap::new();
    data.insert("code", String::from("NOT_FOUND"));
    data.insert("url", uri.path().to_owned());

    (StatusCode::NOT_FOUND, Json(data))
}
