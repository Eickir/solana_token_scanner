use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use crate::platforms::platforms::Platform;

#[derive(Debug)]
pub struct TokenPreflight {
    pub token_address: Pubkey,
    pub platform: Option<Platform>, 
    pub transactions_fetched: usize,
    pub creation_signature: String,
    pub transactions_signatures: Vec<String>,
}

impl TokenPreflight {
    pub fn new(
        token_address: Pubkey,
        platform: Option<Platform>, 
        transactions_fetched: usize,
        creation_signature: String,
        transactions_signatures: Vec<String>,
    ) -> Self {
        Self {
            token_address,
            platform,
            transactions_fetched,
            creation_signature,
            transactions_signatures,
        }
    }
}

impl Display for TokenPreflight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenPreflight(token_address={}, platform={:?}, transactions_fetched={}, creation_signature={}, transactions_signatures=[{:?}]",
            self.token_address,
            self.platform,
            self.transactions_fetched,
            self.creation_signature,
            self.transactions_signatures.join(",")
        )
    }
}
