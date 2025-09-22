use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub creator: Pubkey,
    pub timestamp: u64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
}

#[derive(Debug)]
pub struct TradeEvent {
    pub signature: String, 
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: u64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u16,
    pub fee: u64,
    pub creator: Pubkey,
    pub creator_fee_basis_points: u16,
    pub creator_fee: u64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: u64,
}
