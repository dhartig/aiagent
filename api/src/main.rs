mod db;
mod handlers;
mod models;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database connection pool
    let db_pool = db::create_pool().await;

    // Build our application with routes
    let app = Router::new()
        .route("/get_locations", get(handlers::get_locations))
        .route("/get_average_temp_by_date", get(handlers::get_average_temp_by_date))
        .route("/get_total_precipitation_by_month", get(handlers::get_total_precipitation_by_month))
        .route("/get_yearly_precipitation", get(handlers::get_yearly_precipitation))
        .layer(CorsLayer::permissive())
        .with_state(db_pool);

    // Run the server
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid SERVER_PORT");
    
    let addr = SocketAddr::from((
        host.parse::<std::net::IpAddr>().expect("Invalid SERVER_HOST"),
        port,
    ));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
