use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;
use crate::domain::decoder::error::DecodeError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The address you gave is {0} bytes long. Solana address is always 32 bytes long")]
    WrongSizeToken(usize),
    #[error("The token you want to analyze does not exist. Address used: `{0}`")]
    TokenNotFound(Pubkey),
    #[error(
        "The address you entered is not owned by TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA. Owner: {0}"
    )]
    NotAToken(Pubkey),
    #[error("The token you want to analyze doesn't have any transactions.")]
    NoTransactionRecorded,
    #[error(
        "The token you want to analyze has more than 1000 transactions. {transactions_fetched} Transactions fetched"
    )]
    TooManyTransactions { transactions_fetched: usize },
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Upstream(#[from] ClientError),
}

pub type Result<T> = std::result::Result<T, Error>;
