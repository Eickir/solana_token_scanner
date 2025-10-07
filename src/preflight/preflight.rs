use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::fmt::Display;
use spl_token::ID as token_program_id;

pub enum PreflightVerdict {
    Pass, 
    Fail(Vec<Reason>)
}

impl Display for PreflightVerdict {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => write!(f, "Preflight Verdict: PASS"),  
            Self::Fail(reasons) => write!(f, "Preflight Verdict: FAIL. Reasons: {:?}", reasons),  
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    AddressDoesNotExist,
    AddressIsNotASplMint,              
    NoTransactionRecorded,              
    TooManyTransactions { count: u64 }, 
    CreationSignatureUndetermined,           
}

impl Display for Reason {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressDoesNotExist => write!(f, "Address does not exist"), 
            Self::AddressIsNotASplMint=> write!(f, "Address is not a SPL mint"), 
            Self::NoTransactionRecorded => write!(f, "No transactions recorded"),
            Self::TooManyTransactions{count} => write!(f, "Too many transactions ({count})"),
            Self::CreationSignatureUndetermined =>  write!(f, "Creation signature undetermined"),
        }
    }

}

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
        creation_signature: Option<Signature>
    ) -> Self {
        
        Self {
            address,
            owner,
            tx_count,
            creation_signature,
        }
    }

    pub fn preflight_verdict(&self) -> PreflightVerdict {

        let mut reasons: Vec<Reason> = Vec::new();

        match self.owner {
            Some(owner) if owner == token_program_id => {}
            Some(_) => reasons.push(Reason::AddressIsNotASplMint),
            None => reasons.push(Reason::AddressDoesNotExist),
    }

        match self.tx_count {
            Some(0) => reasons.push(Reason::NoTransactionRecorded),
            Some(n) if n >= 1_000 => reasons.push(Reason::TooManyTransactions { count: n }),
            Some(_) => {}
            None => reasons.push(Reason::NoTransactionRecorded),
        }

        if self.creation_signature.is_none() {
            reasons.push(Reason::CreationSignatureUndetermined);
        }

        if reasons.is_empty() { PreflightVerdict::Pass } else { PreflightVerdict::Fail(reasons) }
    }


}

impl Display for TokenPreflight {


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "TokenPreflight(address= {}, owner= {:?}, tx_count= {:?}, creation_signature= {:?})", 
        self.address, 
        self.owner, 
        self.tx_count, 
        self.creation_signature,
        )
        
    }

}