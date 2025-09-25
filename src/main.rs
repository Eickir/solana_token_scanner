use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{lamports, pubkey::Pubkey};
use std::{collections::HashMap, str::FromStr};
mod domain;
mod error;
mod infra;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
mod services;
use services::preflight::run_analysis;
mod platforms;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status_client_types::UiTransactionEncoding;
use domain::token_stats::TokenStats;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().compact())
        .init();

    let rpc_url = std::env::var("RPC_URL").unwrap();
    let rpc_client = RpcClient::new(rpc_url);

    let token_address_str = "CAeA3EnXgnPrX4YswxKoEowEAdJTZY6AhGGnMNJ3pump";
    let token_address = Pubkey::from_str(token_address_str)
        .map_err(|_| error::Error::WrongSizeToken(token_address_str.as_bytes().len()))?;

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let analysis = run_analysis(&rpc_client, token_address, &config).await;

    if let Err(e) = analysis {
        tracing::error!("❌ {e}");
        std::process::exit(1);
    } 

    let (token_preflight, decoded_trade, decoded_create) = analysis.unwrap();
    println!("CreateEvent: {:?}",decoded_create);
    println!("TokenStats: {:?}", TokenStats::new(&decoded_trade));

    
    Ok(())
}
