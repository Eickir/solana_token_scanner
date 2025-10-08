use crate::preflight::TokenPreflight;
use crate::infra::error::Result;
use crate::scanner::token_scanner::TokenScanner;
use solana_sdk::signature::Signature;
use solana_sdk::pubkey::Pubkey;

pub async fn compute_preflight(scanner: &TokenScanner, address: &Pubkey) -> Result<TokenPreflight> {

    let mut owner: Option<Pubkey> = None;
    let mut tx_count: Option<u64> = None;
    let mut creation_signature: Option<Signature> = None;

    let account = scanner.client.pubkey_account(&address).await?;
    
    if let Some(acc) = account {

        owner = Some(acc.owner);
        let mut sigs = scanner.client.retrieve_pubkey_signatures(&address).await?; 
        tx_count = Some(sigs.len() as u64);

        if tx_count.map_or(false, |n| n < 1_000) {
            creation_signature = sigs.pop();
        }

    }

    Ok(TokenPreflight::new(*address, owner, tx_count, creation_signature))

}