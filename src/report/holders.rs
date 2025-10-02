#[derive(Debug)]
pub struct Holders {
    wallet: Pubkey,
    amount: f64,
    share: f64

}

impl Holders {

    pub fn new(wallet: Pubkey, amount: f64, share: f64) -> Self {
        Self {
            wallet,
            amount,
            share,
        }
    }

}