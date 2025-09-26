use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64, 
}