use axum::{Router, routing::get};
use lion::handlers::index;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static/"))
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
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
