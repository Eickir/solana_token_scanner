use dotenv::dotenv;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcTransactionConfig};
use solana_client::rpc_response;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use solana_transaction_status_client_types::UiTransactionEncoding;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::Mint,
};
use std::time::Instant;
use std::{env, ops::Deref, str::FromStr};
mod error;
use error::Result;
use spl_token::ID;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().compact())
        .init();

    if let Err(e) = run_analysis().await {
        tracing::error!("❌ {e}");
        std::process::exit(1);
    }

    Ok(())

}

async fn fetch_account(rpc_client: &RpcClient, token_address: Pubkey) -> error::Result<Account> {
    let mut account = rpc_client.get_multiple_accounts(&[token_address]).await?;
    account.pop().flatten().ok_or(error::Error::TokenNotFound(token_address))
}

fn ensure_token_is_token_account(account: &Account) -> error::Result<()> {
    match account.owner {
        ID => Ok(()),
        _ => Err(error::Error::NotAToken(account.owner)),
    }
}

async fn preflight_token_check(rpc_client: &RpcClient, token_address: Pubkey) -> error::Result<TokenPreflight> {
    
    let signatures = rpc_client
        .get_signatures_for_address(&token_address)
        .await?;

    match signatures.len() {
        0 => Err(error::Error::NoTransactionRecorded), 
        1..1000 => {
            let tx_sigs: Vec<String> = signatures.into_iter().map(|s| s.signature).collect();
            let creation_signature = tx_sigs.last().cloned().unwrap_or_default();
            Ok(TokenPreflight::new(
                token_address,
                tx_sigs.len(),
                creation_signature,
                tx_sigs))},
        _ => Err(error::Error::TooManyTransactions {
            transactions_fetched: signatures.len(),
        }),
    }
}

async fn token_preflight(rpc_client: &RpcClient, token_address: Pubkey) -> Result<TokenPreflight> {
    let account = fetch_account(rpc_client, token_address).await?;   // Err(TokenNotFound) si absent
    ensure_token_is_token_account(&account)?;                 // Err(NotAToken) si owner KO
    preflight_token_check(rpc_client, token_address).await           // Err(TooManyTransactions | NoTransactionRecorded) ou Ok(TokenPreflight)
}

pub async fn run_analysis() -> error::Result<()> {

    dotenv().ok();
    let rpc_url = std::env::var("RPC_URL").unwrap();
    let rpc_client = RpcClient::new(rpc_url);

    let addr = Pubkey::from_str("DRPcZEDqiA9ppVqJYPmj8kHBBWHLRKoeQJJAc5Ypump").unwrap();
    let preflight = token_preflight(&rpc_client, addr).await?;

    tracing::info!(?preflight, "✅ token prêt pour analyse");
    Ok(())

}

#[derive(Debug)]
struct TokenPreflight {
    token_address: Pubkey,
    transactions_fetched: usize,
    creation_signature: String,
    transactions_signatures: Vec<String>,
}

impl TokenPreflight {
    fn new(
        token_address: Pubkey,
        transactions_fetched: usize,
        creation_signature: String,
        transactions_signatures: Vec<String>,
    ) -> Self {
        Self {
            token_address,
            transactions_fetched,
            creation_signature,
            transactions_signatures,
        }
    }
}
