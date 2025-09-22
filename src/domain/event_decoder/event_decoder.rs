use crate::domain::event_decoder::decode_error::Result;
use crate::platforms::platforms::Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Create,
    Trade,
}

pub trait EventDecoder {
    type Create;
    type Trade;

    fn platform(&self) -> Platform;
    fn classify(&self, payload: &[u8]) -> Option<EventKind>;
    fn decode_create(&self, payload: &[u8]) -> Result<Self::Create>;
    fn decode_trade(&self, signature: &String, payload: &[u8]) -> Result<Self::Trade>;
}
