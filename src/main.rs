use axum::{
    Router,
    http::{HeaderValue, header::CACHE_CONTROL},
    routing::{get, post},
};
use lion::handlers::{index, set_lang_cookie};
use std::env;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

#[tokio::main]
async fn main() {
    let static_content_cache = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=604800"),
    );
    let app_content_cache = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=86400"),
    );
    let app = Router::new()
        .nest_service(
            "/static/fontawesome-free/css/fontawesome.css",
            ServeDir::new("node_modules/@fortawesome/fontawesome-free/css/fontawesome.min.css"),
        )
        .nest_service(
            "/static/fontawesome-free/css/brands.css",
            ServeDir::new("node_modules/@fortawesome/fontawesome-free/css/brands.min.css"),
        )
        .nest_service(
            "/static/fontawesome-free/webfonts",
            ServeDir::new("node_modules/@fortawesome/fontawesome-free/webfonts/"),
        )
        .nest_service("/static/fonts", ServeDir::new("static/fonts"))
        .nest_service("/static/img", ServeDir::new("static/img"))
        .layer(static_content_cache)
        .nest_service("/static/css", ServeDir::new("static/css"))
        .route("/", get(index))
        .layer(app_content_cache)
        .route("/api/set_lang", post(set_lang_cookie));
    let bind_addr = env::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
