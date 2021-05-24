use actix_web::{middleware::Logger, App, HttpServer};
use std::sync::Arc;
use supermarket_management_system::db::{SupermarketRepository, UserRepository};
use supermarket_management_system::*;
use tracing::info;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    info!("Loading connection configuration...");
    let connection_config = config::ConnectionConfig::from_env().expect("Could not load environment config");
    let connection_pool = Arc::new(
        config::ConnectionConfig::create_db_pool(&connection_config)
            .await
            .expect("Could not create a database connection pool"),
    );
    let secret_key = std::env::var("CRYPTO_SERVICE_SECRET_KEY").expect("Secret key env var not found");
    let crypto_service = Arc::new(config::crypto::CryptoService::new(secret_key));

    let user_repository = Arc::new(UserRepository::new(connection_pool.clone()));
    let shop_repository = Arc::new(SupermarketRepository::new(connection_pool.clone()));

    info!("Starting server at http://{}:{}", connection_config.host, connection_config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(connection_pool.clone())
            .data(crypto_service.clone())
            .data(user_repository.clone())
            .data(shop_repository.clone())
            .configure(handlers::init_app_config)
    })
    .bind(format!("{}:{}", connection_config.host, connection_config.port))?
    .run()
    .await
}
