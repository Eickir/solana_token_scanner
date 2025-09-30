use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use solana_transaction_status_client_types::UiCompiledInstruction;
use spl_token::instruction::TokenInstruction;

#[derive(Debug, Clone)]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64, 
}


pub fn is_transfer(account_keys: &Vec<String>, token_program: &String, bonding_curve: &String) -> bool {

    if account_keys.contains(&token_program) && !account_keys.contains(&bonding_curve) {
        true
    } else {
        false
    }

}

pub fn decode_classic_simple_transfer_ui_from_strings(
    account_keys: &Vec<String>,
    ui_ix: &UiCompiledInstruction,
) -> Option<TokenTransfer> {

    let program_id = Pubkey::from_str(account_keys.get(ui_ix.program_id_index as usize)?).ok()?;
    if program_id != spl_token::id() {
        return None;
    }

    // data base58 -> bytes
    let data = bs58::decode(&ui_ix.data).into_vec().ok()?;

    // map indice local -> Pubkey global (parse à la demande)
    let map_acc = |i: usize| -> Option<Pubkey> {
        let key_index = *ui_ix.accounts.get(i)? as usize;
        Pubkey::from_str(account_keys.get(key_index)?).ok()
    };

    match TokenInstruction::unpack(&data).ok()? {
        // [source, destination, authority, (signers...)]
        TokenInstruction::Transfer { amount } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(1)?,
            amount,
        }),
        // [source, mint, destination, authority, (signers...)]
        TokenInstruction::TransferChecked { amount, .. } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(2)?,
            amount,
        }),
        _ => None,
    }
}

pub fn decode_classic_simple_transfer_ui_with_pubkeys(
    account_keys: &[Pubkey],
    ui_ix: &UiCompiledInstruction,
) -> Option<TokenTransfer> {
    // program = Tokenkeg…
    let program_id = *account_keys.get(ui_ix.program_id_index as usize)?;
    if program_id != spl_token::id() {
        return None;
    }

    // data base58 -> bytes
    let data = bs58::decode(&ui_ix.data).into_vec().ok()?;

    // map indice local -> Pubkey global
    let map_acc = |i: usize| -> Option<Pubkey> {
        let key_index = *ui_ix.accounts.get(i)? as usize;
        account_keys.get(key_index).copied()
    };

    match TokenInstruction::unpack(&data).ok()? {
        // [source, destination, authority, (signers...)]
        TokenInstruction::Transfer { amount } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(1)?,
            amount,
        }),
        // [source, mint, destination, authority, (signers...)]
        TokenInstruction::TransferChecked { amount, .. } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(2)?,
            amount,
        }),
        _ => None,
    }
}

pub fn decode_classic_simple_transfer_ui(
    account_keys: &[Pubkey],
    ui_ix: &UiCompiledInstruction,
) -> Option<TokenTransfer> {
    // 1) Vérifier le programme (Tokenkeg…)
    let program_id = *account_keys.get(ui_ix.program_id_index as usize)?;
    if program_id != spl_token::id() {
        return None;
    }

    // 2) Décoder la data base58 -> bytes
    let data = bs58::decode(&ui_ix.data).into_vec().ok()?;

    // Helper: mappe un index local (dans ui_ix.accounts) -> Pubkey global (dans account_keys)
    let map_acc = |i: usize| -> Option<Pubkey> {
        let key_index = *ui_ix.accounts.get(i)? as usize;
        account_keys.get(key_index).copied()
    };

    // 3) Unpack SPL Token instruction et remapper les comptes
    match TokenInstruction::unpack(&data).ok()? {
        // Accounts: [source, destination, authority, (signers...)]
        TokenInstruction::Transfer { amount } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(1)?,
            amount,
        }),
        // Accounts: [source, mint, destination, authority, (signers...)]
        TokenInstruction::TransferChecked { amount, .. } => Some(TokenTransfer {
            from: map_acc(0)?,
            to:   map_acc(2)?,
            amount,
        }),
        _ => None,
    }
}


/// Utilitaire: décode une liste d'UiCompiledInstruction en SimpleTransfer
pub fn decode_many_classic_simple_transfers_ui(
    account_keys: &[Pubkey],
    ui_ixs: &[UiCompiledInstruction],
) -> Vec<TokenTransfer> {
    ui_ixs.iter()
        .filter_map(|ix| decode_classic_simple_transfer_ui(account_keys, ix))
        .collect()
}

pub fn parse_account_keys(strings: &[String]) -> Option<Vec<Pubkey>> {
    strings.iter()
        .map(|s| Pubkey::from_str(s).ok())
        .collect() // Option<Vec<Pubkey>>
}

pub fn decode_many_classic_simple_transfers_ui_from_strings(
    account_keys_str: &Vec<String>,
    ui_ixs: &[UiCompiledInstruction],
) -> Vec<TokenTransfer> {
    if let Some(account_keys) = parse_account_keys(account_keys_str) {
        ui_ixs.iter()
            .filter_map(|ix| decode_classic_simple_transfer_ui_with_pubkeys(&account_keys, ix))
            .collect()
    } else {
        Vec::new()
    }
}