use crate::infra::SolanaRpc;
use crate::preflight::preflight::TokenPreflight;
use crate::infra::error::Result;
use solana_sdk::signature::Signature;
use solana_sdk::pubkey::Pubkey;

pub async fn compute_preflight(rpc: &SolanaRpc, address: &Pubkey) -> Result<TokenPreflight> {

    let mut owner: Option<Pubkey> = None;
    let mut tx_count: Option<u64> = None;
    let mut creation_signature: Option<Signature> = None;

    let account = rpc.pubkey_account(&address).await?;
    
    if let Some(acc) = account {

        owner = Some(acc.owner);
        let mut sigs = rpc.retrieve_pubkey_signatures(&address).await?; 
        tx_count = Some(sigs.len() as u64);

        if tx_count.map_or(false, |n| n < 1_000) {
            creation_signature = sigs.pop();
        }

    }

    Ok(TokenPreflight::new(*address, owner, tx_count, creation_signature))

}