use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use crate::platforms::platforms::Platform;

#[derive(Debug)]
pub struct TokenPreflight {
    pub token_address: Pubkey,
    pub platform: Option<Platform>, 
    pub transactions_to_analyze: Vec<String>,
}

impl TokenPreflight {
    pub fn new(
        token_address: Pubkey,
        platform: Option<Platform>, 
        transactions_to_analyze: Vec<String>,
    ) -> Self {
        Self {
            token_address,
            platform,
            transactions_to_analyze,
        }
    }
}

impl Display for TokenPreflight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenPreflight(token_address={}, platform={:?}, transactions_to_analyze=[{:?}]",
            self.token_address,
            self.platform,
            self.transactions_to_analyze.join(",")
        )
    }
}
