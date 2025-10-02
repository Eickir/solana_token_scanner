use solana_client::client_error::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum RpcError {
    #[error("RPC error: {0}")]
    Upstream(#[from] ClientError),
}

pub type Result<T> = std::result::Result<T, RpcError>;
