use crate::error::{self, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status_client_types::EncodedTransaction;
use solana_transaction_status_client_types::UiMessage;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_client::client_error::ClientError;
use tokio::time::interval;
use tokio::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Interval;
use solana_sdk::signature::Signature;
use indicatif::{ProgressBar, ProgressStyle};
use std::str::FromStr;
use futures::{stream, StreamExt};

pub async fn fetch_account(rpc: &RpcClient, address: Pubkey) -> Result<Account> {
    let mut accounts = rpc.get_multiple_accounts(&[address]).await?;
    accounts
        .pop()
        .flatten()
        .ok_or(error::Error::TokenNotFound(address))
}

pub fn extract_account_keys(tx: &EncodedConfirmedTransactionWithStatusMeta) -> Option<Vec<String>> {
    let enc = &tx.transaction.transaction; // Option<EncodedTransaction>

    match enc {
        EncodedTransaction::Json(ui_tx) => match &ui_tx.message {
            UiMessage::Raw(raw) => Some(raw.account_keys.clone()),
            UiMessage::Parsed(parsed) => Some(
                parsed
                    .account_keys
                    .iter()
                    .map(|k| k.pubkey.clone())
                    .collect(),
            ),
        },
        _ => None, // Legacy/Binary
    }
}

pub async fn retrieve_transactions(
    rpc: &RpcClient,
    signatures: Vec<String>,
    config: RpcTransactionConfig,
) -> std::result::Result<Vec<EncodedConfirmedTransactionWithStatusMeta>, ClientError> {
    const MAX_CONCURRENT: usize = 10;
    const TARGET_RPS: u32 = 20; // plafond global de ton RPC
    let tick_every = Duration::from_millis(1_000 / TARGET_RPS as u64);

    // Crée un intervalle global et partage-le entre futures
    let ticker = Arc::new(Mutex::new(interval(tick_every)));

    // Barre de progression
    let pb = ProgressBar::new(signatures.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("##-"),
    );

    // Création des futures
    let futs = signatures.into_iter().map(|s| {
        let cfg = config.clone();
        let pb = pb.clone();
        let ticker = ticker.clone();

        async move {
            // Attendre un tick global → cadence max = TARGET_RPS
            {
                let mut t: tokio::sync::MutexGuard<'_, Interval> = ticker.lock().await;
                t.tick().await;
            }

            // Parse de la signature
            let sig = Signature::from_str(&s)
                .map_err(|e| ClientError::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            // Appel RPC
            let result = rpc.get_transaction_with_config(&sig, cfg).await;

            pb.inc(1);
            result
        }
    });

    // Exécuter avec parallélisme borné
    let results = stream::iter(futs)
        .buffer_unordered(MAX_CONCURRENT)
        .collect::<Vec<_>>()
        .await;

    pb.finish_with_message("✅ Analyse terminée");

    // Collecter les OK
    let mut out = Vec::new();
    for r in results {
        match r {
            Ok(tx) => out.push(tx),
            Err(e) => tracing::warn!("RPC error: {e}"),
        }
    }

    Ok(out)
}