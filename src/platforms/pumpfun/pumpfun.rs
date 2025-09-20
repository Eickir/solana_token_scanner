use super::events::{CreateEvent, TradeEvent};
use crate::domain::event_decoder::decode_error::Result;
use crate::domain::event_decoder::event_decoder::EventDecoder;
use crate::domain::event_decoder::event_decoder::EventKind;
use crate::domain::event_decoder::helpers::{
    read_bool_u8, read_pubkey, read_string, read_u16_le, read_u64_le,
};
use crate::event_decoder::decode_error::DecodeError;
use crate::platforms::platforms::Platform;

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

        let name = read_string(&mut payload)?;
        let symbol = read_string(&mut payload)?;
        let uri = read_string(&mut payload)?;
        let mint = read_pubkey(&mut payload)?;
        let bonding_curve = read_pubkey(&mut payload)?;
        let user = read_pubkey(&mut payload)?;
        let creator = read_pubkey(&mut payload)?;
        let timestamp = read_u64_le(&mut payload)?;
        let vtok = read_u64_le(&mut payload)?;
        let vsol = read_u64_le(&mut payload)?;
        let rtok = read_u64_le(&mut payload)?;
        let supply = read_u64_le(&mut payload)?;

        Ok(Self::Create {
            name,
            symbol,
            uri,
            mint,
            bonding_curve,
            user,
            creator,
            timestamp,
            virtual_token_reserves: vtok,
            virtual_sol_reserves: vsol,
            real_token_reserves: rtok,
            token_total_supply: supply,
        })
    }

    fn decode_trade(&self, mut payload: &[u8]) -> Result<Self::Trade> {
        if payload.len() < 8 {
            return Err(DecodeError::ShortBuffer("discriminator"));
        }
        payload = &payload[8..];

        let mint = read_pubkey(&mut payload)?;
        let sol_amount = read_u64_le(&mut payload)?;
        let token_amount = read_u64_le(&mut payload)?;
        let is_buy = read_bool_u8(&mut payload)?;
        let user = read_pubkey(&mut payload)?;
        let timestamp = read_u64_le(&mut payload)?;
        let virtual_sol_reserves = read_u64_le(&mut payload)?;
        let virtual_token_reserves = read_u64_le(&mut payload)?;
        let real_sol_reserves = read_u64_le(&mut payload)?;
        let real_token_reserves = read_u64_le(&mut payload)?;
        let fee_recipient = read_pubkey(&mut payload)?;
        let fee_basis_points = read_u16_le(&mut payload)?;
        let fee = read_u64_le(&mut payload)?;
        let creator = read_pubkey(&mut payload)?;
        let creator_fee_basis_points = read_u16_le(&mut payload)?;
        let creator_fee = read_u64_le(&mut payload)?;
        let track_volume = read_bool_u8(&mut payload)?;
        let total_unclaimed_tokens = read_u64_le(&mut payload)?;
        let total_claimed_tokens = read_u64_le(&mut payload)?;
        let current_sol_volume = read_u64_le(&mut payload)?;
        let last_update_timestamp = read_u64_le(&mut payload)?;

        Ok(Self::Trade {
            mint,
            sol_amount,
            token_amount,
            is_buy,
            user,
            timestamp,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,
            fee_recipient,
            fee_basis_points,
            fee,
            creator,
            creator_fee_basis_points,
            creator_fee,
            track_volume,
            total_unclaimed_tokens,
            total_claimed_tokens,
            current_sol_volume,
            last_update_timestamp,
        })
    }
}
