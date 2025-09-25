use crate::platforms::pumpfun::events::TradeEvent;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
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

    // --- Nouveaux champs ---
    pub avg_trades_per_second: Option<f64>,
    pub avg_trades_per_wallet: Option<f64>,
    pub avg_volume_per_wallet_sol: Option<f64>,
    pub avg_volume_per_tx_sol: Option<f64>,
    pub avg_volume_buy_sol: Option<f64>,
    pub avg_volume_sell_sol: Option<f64>,
    pub full_range_len: usize,
    pub seconds_with_trades: usize,
    pub coverage_ratio: f64,
}

impl TokenStats {
    pub fn new(trades: &Vec<TradeEvent>) -> Self {
        // --- agrégats "historiques" (inchangés) ---
        let mut total_trades = 0usize;
        let mut total_lamports: u128 = 0;
        let mut buy_lamports: u128 = 0;
        let mut sell_lamports: u128 = 0;
        let mut buy_count = 0usize;
        let mut sell_count = 0usize;

        let mut makers = HashSet::<Pubkey>::new();
        let mut buyers = HashSet::<Pubkey>::new();
        let mut sellers = HashSet::<Pubkey>::new();

        // --- agrégats pour métriques avancées (1 seule passe) ---
        let mut count_per_timestamp: HashMap<u64, usize> = HashMap::new();
        let mut count_per_wallet: HashMap<Pubkey, usize> = HashMap::new();
        let mut lamports_per_wallet: HashMap<Pubkey, u128> = HashMap::new();
        let mut lamports_per_tx: HashMap<&str, u128> = HashMap::new();
        let mut lamports_per_buy_sig: HashMap<&str, u128> = HashMap::new();
        let mut lamports_per_sell_sig: HashMap<&str, u128> = HashMap::new();
        let (mut min_ts, mut max_ts): (Option<u64>, Option<u64>) = (None, None);

        for t in trades {
            // existant
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

            // nouveaux agrégats
            *count_per_timestamp.entry(t.timestamp).or_default() += 1;
            *count_per_wallet.entry(t.user).or_default() += 1;
            *lamports_per_wallet.entry(t.user).or_default() += t.sol_amount as u128;

            let sig = t.signature.as_str(); // évite clone
            *lamports_per_tx.entry(sig).or_default() += t.sol_amount as u128;
            if t.is_buy {
                *lamports_per_buy_sig.entry(sig).or_default() += t.sol_amount as u128;
            } else {
                *lamports_per_sell_sig.entry(sig).or_default() += t.sol_amount as u128;
            }

            min_ts = Some(min_ts.map_or(t.timestamp, |m| m.min(t.timestamp)));
            max_ts = Some(max_ts.map_or(t.timestamp, |m| m.max(t.timestamp)));
        }

        // --- dérivés (aucune re-itération des trades) ---
        let avg_trades_per_second     = avg_count(&count_per_timestamp);
        let avg_trades_per_wallet     = avg_count(&count_per_wallet);
        let avg_volume_per_wallet_sol = avg_lamports_as_sol(&lamports_per_wallet);
        let avg_volume_per_tx_sol     = avg_lamports_as_sol(&lamports_per_tx);
        let avg_volume_buy_sol        = avg_lamports_as_sol(&lamports_per_buy_sig);
        let avg_volume_sell_sol       = avg_lamports_as_sol(&lamports_per_sell_sig);
        let (full_range_len, seconds_with_trades, coverage_ratio)
            = coverage(&count_per_timestamp, min_ts, max_ts);

        // --- retour : champs historiques inchangés + nouveaux champs ---
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

            // nouveaux
            avg_trades_per_second,
            avg_trades_per_wallet,
            avg_volume_per_wallet_sol,
            avg_volume_per_tx_sol,
            avg_volume_buy_sol,
            avg_volume_sell_sol,
            full_range_len,
            seconds_with_trades,
            coverage_ratio,
        }
    }
}

// -------- Helpers privés au module --------

fn avg_count<K: Eq + Hash>(m: &HashMap<K, usize>) -> Option<f64> {
    if m.is_empty() {
        None
    } else {
        Some(m.values().sum::<usize>() as f64 / m.len() as f64)
    }
}

fn avg_lamports_as_sol<K: Eq + Hash>(m: &HashMap<K, u128>) -> Option<f64> {
    if m.is_empty() {
        None
    } else {
        Some((m.values().sum::<u128>() as f64 / LAMPORTS_PER_SOL) / m.len() as f64)
    }
}

fn coverage(
    count_per_ts: &HashMap<u64, usize>,
    min_ts: Option<u64>,
    max_ts: Option<u64>,
) -> (usize, usize, f64) {
    match (min_ts, max_ts) {
        (Some(min), Some(max)) if max >= min => {
            let full = (max - min + 1) as usize;
            let used = count_per_ts.len();
            (full, used, used as f64 / full as f64)
        }
        _ => (0, 0, 0.0),
    }
}
