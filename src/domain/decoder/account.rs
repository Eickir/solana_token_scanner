use crate::platforms::platforms::Platform;
use solana_sdk::program_pack::Pack;
use spl_token::state::Mint;
use solana_sdk::pubkey::Pubkey;
use super::error::{DecodeError, Result};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    Mint,
    BondingCurve,
}

pub trait AccountDecoder {

    type BondingCurve;

    fn platform(&self) -> Platform;
    fn classify(&self, owner: &Pubkey) -> Option<AccountKind>;
    fn decode_mint_account(&self, account_data: &Vec<u8>) -> Mint {
        Mint::unpack_from_slice(&account_data).expect("Decode Error")
    }
    fn decode_bonding_curve_account(&self, account_data: &Vec<u8>) -> Result<Self::BondingCurve>;

}