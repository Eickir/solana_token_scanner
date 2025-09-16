use crate::infra::solana_rpc::fetch_account;
use crate::{domain::analysis::TokenPreflight, error, error::Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
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

    match signatures.len() {
        0 => Err(error::Error::NoTransactionRecorded),
        1..1000 => {
            let tx_sigs: Vec<String> = signatures.into_iter().map(|s| s.signature).collect();
            let creation_signature = tx_sigs.last().cloned().unwrap_or_default();
            Ok(TokenPreflight::new(
                token_address,
                tx_sigs.len(),
                creation_signature,
                tx_sigs,
            ))
        }
        _ => Err(error::Error::TooManyTransactions {
            transactions_fetched: signatures.len(),
        }),
    }
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
