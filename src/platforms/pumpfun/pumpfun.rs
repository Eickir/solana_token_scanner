use super::events::{CreateEvent, TradeEvent};
use crate::domain::decoder::account::AccountDecoder;
use crate::domain::decoder::error::Result;
use crate::domain::decoder::event::EventDecoder;
use crate::domain::decoder::event::EventKind;
use crate::domain::decoder::helpers::{
    read_bool_u8, read_pubkey, read_string, read_u16_le, read_u64_le,
};
use crate::platforms::pumpfun::events::TradeEventWire;
use std::str::FromStr;
use crate::platforms::pumpfun::accounts::BondingCurve;
use super::super::constants::PUMPFUN_PROGRAM_ID;
use borsh::BorshDeserialize;
use spl_token::ID;
use crate::domain::decoder::error::DecodeError;
use crate::domain::decoder::account::AccountKind;
use crate::platforms::platforms::Platform;
use solana_sdk::pubkey::Pubkey;


pub const CREATE_DISCRIMINATOR: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
pub const TRADE_DISCRIMINATOR: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

pub struct PumpFun;

impl EventDecoder for PumpFun {
    type Create = CreateEvent;
    type Trade = TradeEvent;

    fn platform(&self) -> Platform {
        Platform::PumpFun
    }

    fn classify(&self, payload: &[u8]) -> Option<EventKind> {

        let discriminator: &[u8; 8] = payload.get(..8)?.try_into().ok()?;
        
        match *discriminator {
            CREATE_DISCRIMINATOR => Some(EventKind::Create), 
            TRADE_DISCRIMINATOR => Some(EventKind::Trade), 
            _ => None
        }

    }

    fn decode_create(&self, mut payload: &[u8]) -> Result<Self::Create> {
        if payload.len() < 8 {
            return Err(DecodeError::ShortBuffer("discriminator"));
        }
        payload = &payload[8..];

        let decoded_create = Self::Create::deserialize_reader(&mut payload)?;

        Ok(decoded_create)
           
    }

    fn decode_trade(&self, signature: &String, mut payload: &[u8]) -> Result<Self::Trade> {
        if payload.len() < 8 {
            return Err(DecodeError::ShortBuffer("discriminator"));
        }
        payload = &payload[8..];

        let wire: TradeEventWire = TradeEventWire::deserialize_reader(&mut payload)?;

        Ok((signature.to_string(), wire).into())

    }

}

impl AccountDecoder for PumpFun {

    type BondingCurve = BondingCurve;
    
    fn platform(&self) -> Platform {
        Platform::PumpFun
    }

    fn classify(&self, owner: &Pubkey) -> Option<AccountKind> {

        let pump_program = Pubkey::from_str(PUMPFUN_PROGRAM_ID).expect("Wrong address");

        match *owner {
            x if x == pump_program => Some(AccountKind::BondingCurve), 
            ID => Some(AccountKind::Mint), 
            _ => {
                None
            }

        } 
        
    }

    fn decode_bonding_curve_account(&self, account_data: &Vec<u8>) -> Result<Self::BondingCurve> {

        if account_data.len() < 8 {
                return Err(DecodeError::ShortBuffer("anchor discriminator"));
            }
            let mut cursor = &account_data[8..];

            let bc = BondingCurve::deserialize_reader(&mut cursor)?;

            Ok(bc)
        
    }

}