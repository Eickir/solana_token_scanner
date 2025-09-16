use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
mod domain;
mod error;
mod infra;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
mod services;
use services::preflight::run_analysis;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().compact())
        .init();

    let rpc_url = std::env::var("RPC_URL").unwrap();
    let rpc_client = RpcClient::new(rpc_url);

    let token_address_str = "DRPcZEDqiA9ppVqJYPmj8kHBBWHLRKoeQJJAc5Ypump";
    let token_address = 
        Pubkey::from_str(token_address_str).map_err(|_| {
            error::Error::WrongSizeToken(
                token_address_str
                    .as_bytes()
                    .len(),
            )
        })?;

    if let Err(e) = run_analysis(&rpc_client, token_address).await {
        tracing::error!("❌ {e}");
        std::process::exit(1);
    }

    Ok(())
}
