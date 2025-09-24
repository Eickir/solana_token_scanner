use crate::platforms::pumpfun::events::TradeEvent;
use std::collections::HashSet;
use solana_sdk::pubkey::Pubkey;

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

#[derive(Debug)]
pub struct TokenStats {
    pub total_trades: usize,
    pub total_volume_sol: f64,
    pub makers_count: usize,
    pub buy_count: usize,
    pub sell_count: usize,
    pub buy_volume_sol: f64,
    pub sell_volume_sol: f64,
    pub buyers_count: usize,
    pub sellers_count: usize,
}

impl TokenStats {

    pub fn new(trades: &Vec<TradeEvent>) -> Self {

        let mut total_trades = 0usize;
            let mut total_lamports: u128 = 0;
            let mut buy_lamports: u128 = 0;
            let mut sell_lamports: u128 = 0;
            let mut buy_count = 0usize;
            let mut sell_count = 0usize;

            let mut makers = HashSet::<Pubkey>::new();
            let mut buyers = HashSet::<Pubkey>::new();
            let mut sellers = HashSet::<Pubkey>::new();

            for t in trades {
                total_trades += 1;
                total_lamports += t.sol_amount as u128;
                makers.insert(t.user);

                if t.is_buy {
                    buy_count += 1;
                    buy_lamports += t.sol_amount as u128;
                    buyers.insert(t.user);
                } else {
                    sell_count += 1;
                    sell_lamports += t.sol_amount as u128;
                    sellers.insert(t.user);
                }
            }

            Self {
                total_trades,
                total_volume_sol: (total_lamports as f64) / LAMPORTS_PER_SOL,
                makers_count: makers.len(),
                buy_count,
                sell_count,
                buy_volume_sol: (buy_lamports as f64) / LAMPORTS_PER_SOL,
                sell_volume_sol: (sell_lamports as f64) / LAMPORTS_PER_SOL,
                buyers_count: buyers.len(),
                sellers_count: sellers.len(),
            }

    }

}