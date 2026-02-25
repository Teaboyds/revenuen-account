use revenue_service::{
    config::config_loader::load,
    infrastructure::postgres::postgres_connection::establish_connection,
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let env = match load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load env {:?}", e);
            std::process::exit(1)
        }
    };

    info!("ENV has been loaded");

    let _ = match establish_connection(&env.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connected database {:?}", e);
            std::process::exit(1)
        }
    };

    info!("Establish Database Successfully");
}
