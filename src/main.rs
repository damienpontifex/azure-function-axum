use std::env;
use std::net::Ipv4Addr;
use axum::{routing::{get, post}, Router, http::StatusCode};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
mod functions;

#[tokio::main]
async fn main() {
    let port: u16 = env::var("FUNCTIONS_CUSTOMHANDLER_PORT").ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

    tracing_subscriber::fmt::init();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    let app = Router::new()
        .route("/", get(|| async { StatusCode::OK }))
        .route("/api/HttpTrigger", get(functions::greet_handler))
        .route("/MyTimer", post(functions::my_timer))
        .layer(middleware_stack);

    axum::Server::bind(&(Ipv4Addr::UNSPECIFIED, port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
