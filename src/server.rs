use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::put;
use tower_http::services::{ServeDir, ServeFile};

use crate::switcher::Connection;

type HandlerResponse = Result<&'static str, (StatusCode, String)>;

pub async fn start_server(connection: Connection<SocketAddr>, port: u16) {
    let connection_ref = Arc::new(connection);
    let connection_ref_set = connection_ref.clone();
    let connection_ref_reset = connection_ref.clone();
    let connection_ref_save = connection_ref.clone();
    let connection_ref_load = connection_ref.clone();
    let front_base_url = std::env::var("SCREEN_SWITCHER_BASE_URL").unwrap_or("/".to_string());

    let serve_dir = ServeDir::new(&(front_base_url.clone()))
        .not_found_service(ServeFile::new(&(front_base_url.clone() + "/index.html")));
    let app = Router::new()
        .route("/set", put(move |Query(params): Query<HashMap<String, String>>| async move { 
            let from = get_number(&params, "from").map_err(map_err)?;
            let to = get_number(&params, "to").map_err(map_err)?;
            connection_ref_set.set(from, to);
            HandlerResponse::Ok("ok")
        }))
        .route("/reset", put(move || async move {
            connection_ref_reset.reset();
            "ok"
        }))
        .route("/save", put(move |Query(params): Query<HashMap<String, String>>| async move { 
            let pos = get_number(&params, "pos").unwrap_or(1);
            connection_ref_save.save(pos);
            "ok"
        }))
        .route("/load", put(move |Query(params): Query<HashMap<String, String>>| async move { 
            let pos = get_number(&params, "pos").unwrap_or(1);
            connection_ref_load.load(pos);
            "ok"
        }))
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.expect("Start server failed.");
    axum::serve(listener, app).await.expect("Run server failed.");
}

fn get_number(query: &HashMap<String, String>, key: &str) -> Result<u8, String> {
    if let Some(from) = query.get(key) {
        if let Ok(from) = from.parse() {
            Ok(from)
        } else {
            Err(format!("Not a legal number in key {}", key))
        }
    } else {
        Err(format!("Missing key {}", key))
    }
}

fn map_err(err: String) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, err)
}
