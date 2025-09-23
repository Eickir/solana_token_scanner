use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
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

#[derive(Debug, BorshDeserialize)]
pub struct TradeEventWire {
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

impl From<(String, TradeEventWire)> for TradeEvent {
    fn from((signature, w): (String, TradeEventWire)) -> Self {
        TradeEvent {
            signature,
            mint: w.mint,
            sol_amount: w.sol_amount,
            token_amount: w.token_amount,
            is_buy: w.is_buy,
            user: w.user,
            timestamp: w.timestamp,
            virtual_sol_reserves: w.virtual_sol_reserves,
            virtual_token_reserves: w.virtual_token_reserves,
            real_sol_reserves: w.real_sol_reserves,
            real_token_reserves: w.real_token_reserves,
            fee_recipient: w.fee_recipient,
            fee_basis_points: w.fee_basis_points,
            fee: w.fee,
            creator: w.creator,
            creator_fee_basis_points: w.creator_fee_basis_points,
            creator_fee: w.creator_fee,
            track_volume: w.track_volume,
            total_unclaimed_tokens: w.total_unclaimed_tokens,
            total_claimed_tokens: w.total_claimed_tokens,
            current_sol_volume: w.current_sol_volume,
            last_update_timestamp: w.last_update_timestamp,
        }
    }
}
