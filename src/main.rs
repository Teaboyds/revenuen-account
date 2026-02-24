use revenue_service::config::config_loader::load;

#[tokio::main]
async fn main() {
    let dotenvy_load = load();
    println!("{:?}", dotenvy_load)
}
