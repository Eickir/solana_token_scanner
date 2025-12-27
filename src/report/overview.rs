use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct Overview {
    pub name: String, 
    pub symbol: String, 
    pub supply: f64,
    pub creator: Pubkey, 
    pub creator_balance: f64,
    pub market_cap_sol: f64,
    pub holders: u64, 
    pub mint_authority: Option<Pubkey>, 
    pub token_locked: Option<f64> 
}

impl Overview {

    pub fn new(name: String, symbol: String, supply: f64, creator: Pubkey, creator_balance: f64, market_cap_sol: f64, holders: u64, mint_authority: Option<Pubkey>, token_locked: Option<f64>) -> Self {
        
        Self {
            name,
            symbol,
            supply,
            creator,
            creator_balance,
            market_cap_sol,
            holders,
            mint_authority,
            token_locked,
        }
    }

}