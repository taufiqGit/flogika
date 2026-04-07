use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

fn is_indonesia(headers: &HeaderMap) -> bool {
    let country = headers
        .get("x-vercel-ip-country")
        .or_else(|| headers.get("cf-ipcountry"))
        .or_else(|| headers.get("x-country-code"))
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim().to_uppercase());

    if matches!(country.as_deref(), Some("ID")) {
        return true;
    }

    let accept_language = headers
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    accept_language.split(',').any(|part| {
        let lang = part.split(';').next().unwrap_or("").trim().to_lowercase();
        lang == "id" || lang.starts_with("id-")
    })
}

async fn root_handler(headers: HeaderMap) -> Response {
    if is_indonesia(&headers) {
        return Redirect::temporary("/id").into_response();
    }

    match tokio::fs::read_to_string("dist/index.html").await {
        Ok(content) => Html(content).into_response(),
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "Missing dist/index.html").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8081);

    let app = Router::new()
        .route("/", get(root_handler))
        .fallback_service(ServeDir::new("dist").append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server exited with errors");
}
