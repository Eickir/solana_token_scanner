use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct ContractAnalysis {
    mint_authority: Option<Pubkey>,
    freeze_authority: Option<Pubkey>,
    is_mintable: bool, 
    is_freezable: bool

}

impl ContractAnalysis {

    pub fn new(mint_authority: Option<Pubkey>, freeze_authority: Option<Pubkey>, is_mintable: bool, is_freezable: bool) -> Self {
        Self {
            mint_authority,
            freeze_authority,
            is_mintable,
            is_freezable,
        }
    }

}