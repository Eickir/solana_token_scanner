use std::convert::TryFrom;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use spl_token::ID as TOKEN_PROGRAM_ID;
use super::{PreflightVerdict, Reason};


#[derive(Debug)]
pub struct TokenPreflight {
    pub address: Pubkey,
    pub owner: Option<Pubkey>,
    pub tx_count: Option<u64>,
    pub creation_signature: Option<Signature>,
}

impl TokenPreflight {
    pub fn new(
        address: Pubkey,
        owner: Option<Pubkey>,
        tx_count: Option<u64>,
        creation_signature: Option<Signature>,
    ) -> Self {
        Self { address, owner, tx_count, creation_signature }
    }

    /// Politique **pure**: calcule le verdict à partir des faits.
    pub fn verdict(&self) -> PreflightVerdict {
        let mut reasons = Vec::new();

        // 1) Owner (SPL Token attendu)
        match self.owner {
            Some(owner) if owner == TOKEN_PROGRAM_ID => {},
            Some(_) => reasons.push(Reason::AddressIsNotASplMint),
            None    => reasons.push(Reason::AddressDoesNotExist),
        }

        // 2) Compte de transactions
        match self.tx_count {
            Some(0)                 => reasons.push(Reason::NoTransactionRecorded),
            Some(n) if n >= 1000 => reasons.push(Reason::TooManyTransactions { count: n }),
            Some(_)                 => {},
            None                    => reasons.push(Reason::TxCountUnknown),
        }

        // 3) Création
        if self.creation_signature.is_none() {
            reasons.push(Reason::CreationSignatureUndetermined);
        }

        if reasons.is_empty() { PreflightVerdict::Pass } else { PreflightVerdict::Fail(reasons) }
    }
}

#[derive(Debug, Clone)]
pub struct PreflightOk {
    pub address: Pubkey,
    pub owner: Pubkey,
    pub tx_count: u64,
    pub creation_signature: Signature,
}

impl TryFrom<TokenPreflight> for PreflightOk {
    type Error = Vec<Reason>; 

    fn try_from(pf: TokenPreflight) -> Result<Self, Self::Error> {
        match pf.verdict() {
            PreflightVerdict::Pass => Ok(Self {
                address: pf.address,
                owner: pf.owner.expect("checked by verdict"),
                tx_count: pf.tx_count.expect("checked by verdict"),
                creation_signature: pf.creation_signature.expect("checked by verdict"),
            }),
            PreflightVerdict::Fail(reasons) => Err(reasons),
        }
    }
}
