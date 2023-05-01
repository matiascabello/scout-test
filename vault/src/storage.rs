use soroban_sdk::{Address, BytesN, Env};

use crate::{
    token,
    types::{DataKey, Error},
};

pub fn get_contract_addr(e: &Env) -> Address {
    e.current_contract_address()
}

pub fn put_tot_supply(e: &Env, supply: i128) {
    let key = DataKey::TotSupply;
    e.storage().set(&key, &supply);
}

pub fn get_tot_supply(e: &Env) -> i128 {
    let key = DataKey::TotSupply;
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn write_total_deposited(e: &Env, amount: i128) {
    let key = DataKey::TotalDeposited;
    e.storage().set(&key, &amount);
}

pub fn read_total_deposited(e: &Env) -> i128 {
    let key = DataKey::TotalDeposited;
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn write_balance(e: &Env, addr: Address, balance: i128) {
    let key = DataKey::Balance(addr);
    e.storage().set(&key, &balance);
}

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn remove_balance(e: &Env, addr: Address) {
    let key = DataKey::Balance(addr);
    e.storage().remove(&key)
}

pub fn write_fee_per_share_particular(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::FeePerShareParticular(addr);
    e.storage().set(&key, &amount);
}

pub fn read_fee_per_share_particular(e: &Env, addr: Address) -> i128 {
    let key = DataKey::FeePerShareParticular(addr);
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn remove_fee_per_share_particular(e: &Env, addr: Address) {
    let key = DataKey::FeePerShareParticular(addr);
    e.storage().remove(&key)
}

pub fn write_matured_fees_particular(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::MaturedFeesParticular(addr);
    e.storage().set(&key, &amount);
}

pub fn read_matured_fees_particular(e: &Env, addr: Address) -> i128 {
    let key = DataKey::MaturedFeesParticular(addr);
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn remove_matured_fees_particular(e: &Env, addr: Address) {
    let key = DataKey::MaturedFeesParticular(addr);
    e.storage().remove(&key)
}

pub fn put_fee_per_share_universal(e: &Env, last_recorded: i128) {
    let key = DataKey::FeePerShareUniversal;
    e.storage().set(&key, &last_recorded);
}

pub fn get_fee_per_share_universal(e: &Env) -> i128 {
    let key = DataKey::FeePerShareUniversal;
    e.storage().get(&key).unwrap_or(Ok(0)).unwrap()
}

pub fn put_token_id(e: &Env, token_id: BytesN<32>) {
    let key = DataKey::TokenId;
    e.storage().set(&key, &token_id);
}

pub fn get_token_id(e: &Env) -> BytesN<32> {
    let key = DataKey::TokenId;
    e.storage().get(&key).unwrap().unwrap()
}

pub fn put_flash_loan(e: &Env, id: Address) {
    let key = DataKey::FlashLoan;
    e.storage().set(&key, &id);
}

pub fn get_flash_loan(e: &Env) -> Address {
    let key = DataKey::FlashLoan;
    e.storage().get(&key).unwrap().unwrap()
}

pub fn put_flash_loan_bytes(e: &Env, id: BytesN<32>) {
    let key = DataKey::FlashLoanB;
    e.storage().set(&key, &id);
}

pub fn get_flash_loan_bytes(e: &Env) -> BytesN<32> {
    let key = DataKey::FlashLoanB;
    e.storage().get(&key).unwrap().unwrap()
}

// should be deprecated
pub fn get_token_balance(e: &Env, client: &token::Client) -> i128 {
    client.balance(&get_contract_addr(e)) + client.balance(&get_flash_loan(e))
}

pub fn _transfer_in_vault(e: &Env, from: &Address, amount: &i128) {
    let client = token::Client::new(e, &get_token_id(e));
    let vault_addr = get_contract_addr(e);

    client.xfer(from, &vault_addr, amount);
}

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: Address) {
    let key = DataKey::Admin;
    e.storage().set(&key, &id);
}

pub fn auth_admin(e: &Env, admin: Address) -> Result<(), Error> {
    if read_admin(e) != admin {
        return Err(Error::InvalidAdminAuth);
    }
    admin.require_auth();
    Ok(())
}
