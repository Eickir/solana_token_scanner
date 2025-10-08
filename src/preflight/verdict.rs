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
    TxCountUnknown,           
    TooManyTransactions { count: u64 }, 
    CreationSignatureUndetermined,           
}

impl Display for Reason {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressDoesNotExist => write!(f, "Address does not exist"), 
            Self::AddressIsNotASplMint=> write!(f, "Address is not a SPL mint"), 
            Self::TxCountUnknown=> write!(f, "Transaction count unknown"),
            Self::NoTransactionRecorded => write!(f, "No transactions recorded"),
            Self::TooManyTransactions{count} => write!(f, "Too many transactions ({count})"),
            Self::CreationSignatureUndetermined =>  write!(f, "Creation signature undetermined"),
        }
    }

}
