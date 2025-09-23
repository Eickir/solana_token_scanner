use crate::infra::solana_rpc::extract_account_keys;
use crate::infra::solana_rpc::fetch_account;
use crate::platforms::pumpfun::events::CreateEvent;
use crate::{domain::analysis::TokenPreflight, error, error::Result};
use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_sdk::{account::Account, pubkey::Pubkey};
use solana_transaction_status_client_types::UiTransactionEncoding;
use spl_token::ID;
use crate::domain::decoder::helpers::extract_logs;
use crate::platforms::pumpfun::events::TradeEvent;
use crate::platforms::platforms::Platform;
use std::str::FromStr;
use crate::platforms::pumpfun::pumpfun::PumpFun;
use solana_transaction_status_client_types::EncodedTransaction;
use crate::domain::decoder::event::EventDecoder;
use crate::domain::decoder::event::EventKind;
use crate::platforms::utils::identify_platform;
use crate::infra::solana_rpc::retrieve_transactions;

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

    let tx_sigs: Vec<String> = signatures.into_iter().filter(|signature| signature.err.is_none()).map(|s| s.signature).collect();
    let last_element =  tx_sigs.len().saturating_sub(100);
    let mut last_100: Vec<String> = tx_sigs[last_element..].to_vec();
    last_100.reverse();
    

    let creation_signature = tx_sigs.last().cloned().unwrap_or_default();

    let creation_sig = Signature::from_str(&creation_signature).map_err(|e| {
        error::Error::Upstream(ClientError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            e,
        )))
    })?;

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
            return Err(error::Error::Upstream(ClientError::from(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "unable to extract account_keys (non-Raw message?)",
                ),
            )));
        }
    };

    let platform = identify_platform(&accounts);

    Ok(TokenPreflight::new(
        token_address,
        platform,
        last_100,
    ))
}

async fn token_preflight(rpc_client: &RpcClient, token_address: Pubkey) -> Result<TokenPreflight> {
    let account = fetch_account(rpc_client, token_address).await?;
    ensure_token_is_token_account(&account)?;
    preflight_token_check(rpc_client, token_address).await
}

pub async fn run_analysis(rpc_client: &RpcClient, token_address: Pubkey, config: &RpcTransactionConfig,) -> error::Result<(TokenPreflight, Vec<TradeEvent>, Vec<CreateEvent>)> {
    
    let preflight = token_preflight(&rpc_client, token_address).await?;
    tracing::info!(%preflight, "✅ token prêt pour analyse");
    
    let txs = retrieve_transactions(
        &rpc_client,
        preflight.transactions_to_analyze.clone(),
        *config,
    ).await?;

    let mut decoded_create: Vec<CreateEvent> = Vec::new();
    let mut decoded_trade: Vec<TradeEvent> = Vec::new();

    match preflight.platform {
        Some(Platform::PumpFun) => {
            let my_platform = PumpFun;

            for tx in &txs {
                // a) Récupérer la signature lisible (utile à logguer/attacher au TradeEvent)
                let signature = match &tx.transaction.transaction {
                    EncodedTransaction::Json(inner) => inner
                        .signatures
                        .first()
                        .map(|s| s.to_string())
                        .unwrap_or_default(),
                    _ => String::new(),
                };

                // b) Extraire les blobs logs “Program data:” (Vec<Vec<u8>>)
                if let Some(blobs) = extract_logs(tx) {
                    for blob in blobs {
                        // Selon ton impl: si `classify` attend le discriminant, sépare 8+payload ici.
                        // Sinon, si elle gère le blob complet (disc en tête), passe `&blob` direct.
                        if let Some(kind) = my_platform.classify(&blob) {
                            match kind {
                                EventKind::Create => {
                                    // Utile si tu veux aucher le mint/creator au TGE
                                    let create = my_platform.decode_create(&blob)?;
                                    if create.mint == token_address {
                                        decoded_create.push(create);
                                    }
                                }
                                EventKind::Trade => {
                                    // Ici ta signature est disponible si ton decode_trade en a besoin
                                    let trade = my_platform.decode_trade(&signature, &blob)?;
                                    if trade.mint == token_address {
                                        decoded_trade.push(trade);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
        }
    }

    tracing::info!("✅ token prêt pour analyse: {} trades décodés", decoded_trade.len());
    Ok((preflight, decoded_trade, decoded_create))

}
