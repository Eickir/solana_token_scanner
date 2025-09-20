use crate::error::{self, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status_client_types::EncodedTransaction;
use solana_transaction_status_client_types::UiMessage;

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