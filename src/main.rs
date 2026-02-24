use revenue_service::config::config_loader::load;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _ = match load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load env {:?}", e);
            std::process::exit(1)
        }
    };

    info!("ENV has been loaded");
}
