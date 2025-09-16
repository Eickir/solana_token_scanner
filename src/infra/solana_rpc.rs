use crate::error::{self, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};

pub async fn fetch_account(rpc: &RpcClient, address: Pubkey) -> Result<Account> {
    let mut accounts = rpc.get_multiple_accounts(&[address]).await?;
    accounts
        .pop()
        .flatten()
        .ok_or(error::Error::TokenNotFound(address))
}
