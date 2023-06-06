use soroban_sdk::{token, Address, Env};

use crate::types::DataKey;

pub(crate) fn is_initialized(e: &Env) -> bool {
    e.storage().has(&DataKey::TokenId)
}

pub(crate) fn set_token(e: &Env, id: Address) {
    e.storage().set(&DataKey::TokenId, &id);
}

pub(crate) fn get_token_id(e: &Env) -> Address {
    // safe since we only reach this unwrap when the contract is initialized
    e.storage().get(&DataKey::TokenId).unwrap().unwrap()
}

pub(crate) fn _get_token_balance(e: &Env) -> i128 {
    let token_id: Address = get_token_id(e);
    let client = token::Client::new(e, &token_id);

    client.balance(&e.current_contract_address())
}

pub(crate) fn _has_lp(e: &Env) -> bool {
    e.storage().has(&DataKey::LP)
}

pub(crate) fn set_lp(e: &Env, id: Address) {
    e.storage().set(&DataKey::LP, &id);
}

pub(crate) fn get_lp(e: &Env) -> Address {
    e.storage().get(&DataKey::LP).unwrap().unwrap()
}

pub(crate) fn _remove_lp(e: &Env) {
    e.storage().remove(&DataKey::LP)
}
