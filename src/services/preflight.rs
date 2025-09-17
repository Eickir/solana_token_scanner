use crate::infra::solana_rpc::fetch_account;
use crate::{domain::analysis::TokenPreflight, error, error::Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use solana_sdk::signature::Signature;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_transaction_status_client_types::UiTransactionEncoding;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::client_error::ClientError;
use std::str::FromStr;
use crate::infra::solana_rpc::extract_account_keys;
use crate::identify_platform;
use spl_token::ID;

fn ensure_token_is_token_account(account: &Account) -> error::Result<()> {
    match account.owner {
        ID => Ok(()),
        _ => Err(error::Error::NotAToken(account.owner)),
    }
}

async fn preflight_token_check(
    rpc_client: &RpcClient,
    token_address: Pubkey,
) -> error::Result<TokenPreflight> {

    let signatures = rpc_client
        .get_signatures_for_address(&token_address)
        .await?;

    if signatures.is_empty() {
        return Err(error::Error::NoTransactionRecorded);
    }
    if signatures.len() >= 1000 {
        return Err(error::Error::TooManyTransactions {
            transactions_fetched: signatures.len(),
        });
    }

    let tx_sigs: Vec<String> = signatures.into_iter().map(|s| s.signature).collect();

    let creation_signature = tx_sigs
        .last()
        .cloned()
        .unwrap_or_default();

    let creation_sig = Signature::from_str(&creation_signature)
        .map_err(|e| error::Error::Upstream(ClientError::from(std::io::Error::new(std::io::ErrorKind::Other, e))))?;

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0), // laisse None pour accepter v0+
    };

    let creation_tx = rpc_client
        .get_transaction_with_config(&creation_sig, config)
        .await?;

    let accounts = match extract_account_keys(&creation_tx) {
        Some(keys) => keys,
        None => {
            return Err(error::Error::Upstream(ClientError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unable to extract account_keys (non-Raw message?)",
            ))));
        }
    };

    let platform = identify_platform(&accounts);

    Ok(TokenPreflight::new(
        token_address,
        platform,                 
        tx_sigs.len(),
        creation_signature,
        tx_sigs,
    ))
}


async fn token_preflight(rpc_client: &RpcClient, token_address: Pubkey) -> Result<TokenPreflight> {
    let account = fetch_account(rpc_client, token_address).await?;
    ensure_token_is_token_account(&account)?;
    preflight_token_check(rpc_client, token_address).await
}

pub async fn run_analysis(rpc_client: &RpcClient, token_address: Pubkey) -> error::Result<()> {
    let preflight = token_preflight(&rpc_client, token_address).await?;
    tracing::info!(%preflight, "✅ token prêt pour analyse");
    Ok(())
}
