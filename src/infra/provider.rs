use crate::infra::error::Result;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcTransactionConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_sdk::{commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Account;
use solana_transaction_status_client_types::UiTransactionEncoding;
use solana_sdk::account::Account as SolanaAccount;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;

fn default_tx_config() -> RpcTransactionConfig {
    RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),
    }
}

pub(crate) struct SolanaRpc {
    client: RpcClient,
}

impl SolanaRpc {

    pub fn new(url: impl Into<String>) -> Self {
        let client = RpcClient::new(url.into());
        Self { client }
    }

    pub fn new_with_commitment(url: impl Into<String>, commitment: CommitmentConfig) -> Self {
        let client = RpcClient::new_with_commitment(url.into(), commitment);
        Self { client }
    }

    pub async fn pubkey_account(&self, pubkey: &Pubkey) -> Result<Option<SolanaAccount>> {
        let mut accounts = self.client.get_multiple_accounts(&[*pubkey]).await?;
        Ok(accounts.pop().flatten())
    }

    pub async fn retrieve_pubkey_signatures(&self, pubkey: &Pubkey) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>> {

        let transactions = self.client.get_signatures_for_address(&pubkey).await?;
        Ok(transactions.into_iter().filter_map(|transaction| match transaction.err {
            Some(_err) => None, 
            None => Some(transaction)
        })
        .collect::<Vec<RpcConfirmedTransactionStatusWithSignature>>())

    }

    pub async fn get_transaction(&self, signature: Signature) -> Result<EncodedConfirmedTransactionWithStatusMeta> {

        let config = default_tx_config();
        Ok(self.client.get_transaction_with_config(&signature, config).await?)

    }

    pub async fn mint_token_holders(&self, mint: &Pubkey) -> Result<HashMap<Pubkey, u64>> {
        let memcmp = RpcFilterType::Memcmp(Memcmp::new(
            0,
            MemcmpEncodedBytes::Bytes(mint.to_bytes().to_vec()),
        ));
        let config = RpcProgramAccountsConfig {
            filters: Some(vec![RpcFilterType::DataSize(165), memcmp]),
            account_config: RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                data_slice: None,
                commitment: Some(CommitmentConfig::processed()),
                min_context_slot: None,
            },
            with_context: Some(false),
            sort_results: Some(true),
        };

        let accounts = self
            .client
            .get_program_accounts_with_config(&spl_token::id(), config)
            .await?;

        let mut holders: HashMap<Pubkey, u64> = HashMap::new();

        for (_pubkey, account) in accounts {
            if let Ok(token_acc) = Account::unpack(&account.data) {
                // token_acc.amount est en unités "brutes" (u64). Filtrez > 0.
                if token_acc.amount > 0 {
                    holders.insert(token_acc.owner, token_acc.amount);
                }
            }
        }

        Ok(holders)
    }
}
