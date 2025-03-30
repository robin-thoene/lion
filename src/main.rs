use axum::{
    BoxError, Router,
    http::{HeaderValue, header::CACHE_CONTROL},
    routing::{get, post},
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use lion::handlers::{fallback, health, index, set_lang_cookie};
use std::env;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;
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
            "/static/fontawesome-free/css/all.min.css",
            ServeDir::new("node_modules/@fortawesome/fontawesome-free/css/all.min.css"),
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
        .route("/api/set_lang", post(set_lang_cookie))
        .fallback(fallback)
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .route("/api/health", get(health));
    let bind_addr = env::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
