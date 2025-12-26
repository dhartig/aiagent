use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub type DbPool = Pool;

pub async fn create_pool() -> DbPool {
    let mut cfg = Config::new();
    
    // Configure database connection from environment variables
    cfg.host = Some(std::env::var("DB_HOST").unwrap_or_else(|_| "sayulita.local".to_string()));
    cfg.port = Some(
        std::env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .expect("Invalid DB_PORT")
    );
    cfg.dbname = Some(std::env::var("DB_NAME").unwrap_or_else(|_| "mcpdb".to_string()));
    cfg.user = std::env::var("DB_USER").ok();
    cfg.password = std::env::var("DB_PASSWORD").ok();
    
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create pool")
}
