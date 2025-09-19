use crate::domain::event_decoder::decode_error::{self, DecodeError};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status_client_types::option_serializer::OptionSerializer;
use base64::decode;

pub fn extract_logs(tx: &EncodedConfirmedTransactionWithStatusMeta) -> Option<Vec<Vec<u8>>> {

    match tx.transaction.meta.as_ref() {
        Some(meta) => {
            if let OptionSerializer::Some(log) = meta.log_messages.as_ref() {
                let interesting_log = log
                    .iter()
                    .filter_map(|log| {
                        log.strip_prefix("Program data: ")
                            .and_then(|data| decode(data).ok())
                    })
                    .collect::<Vec<Vec<u8>>>();
                return Some(interesting_log);
            } else {
                None
            }
        }, 
        None => None
    }

}

pub fn read_u16_le(input: &mut &[u8]) -> decode_error::Result<u16> {
    if input.len() < 2 {
        return Err(DecodeError::ShortBuffer("u16"));
    }
    let v = u16::from_le_bytes(input[..2].try_into().unwrap());
    *input = &input[2..];
    Ok(v)
}

pub fn read_u32_le(input: &mut &[u8]) -> decode_error::Result<u32> {
    if input.len() < 4 {
        return Err(DecodeError::ShortBuffer("u32"));
    }
    let v = u32::from_le_bytes(input[..4].try_into().unwrap());
    *input = &input[4..];
    Ok(v)
}

pub fn read_u64_le(input: &mut &[u8]) -> decode_error::Result<u64> {
    if input.len() < 8 {
        return Err(DecodeError::ShortBuffer("u64"));
    }
    let v = u64::from_le_bytes(input[..8].try_into().unwrap());
    *input = &input[8..];
    Ok(v)
}

pub fn read_pubkey(input: &mut &[u8]) -> decode_error::Result<Pubkey> {
    if input.len() < 32 {
        return Err(DecodeError::ShortBuffer("pubkey"));
    }
    let k = Pubkey::new_from_array(input[..32].try_into().unwrap());
    *input = &input[32..];
    Ok(k)
}

pub fn read_bool_u8(input: &mut &[u8]) -> decode_error::Result<bool> {
    if input.len() < 1 {
        return Err(DecodeError::ShortBuffer("bool"));
    }
    let b = input[0];
    *input = &input[1..];
    Ok(b != 0)
}

pub fn read_string(input: &mut &[u8]) -> decode_error::Result<String> {
    let len = read_u32_le(input)? as usize;
    if input.len() < len {
        return Err(DecodeError::ShortBuffer("string bytes"));
    }
    let bytes = input[..len].to_vec();
    *input = &input[len..];
    String::from_utf8(bytes).map_err(DecodeError::Utf8)
}
