use crate::{
    flash_loan,
    storage::*,
    token,
    types::{BatchKey, BatchObj, DataKey},
};
use soroban_sdk::{contractimpl, log, Address, BytesN, Env, Vec};

pub trait VaultContractTrait {
    fn initialize(
        e: Env,
        admin: Address,
        token_id: BytesN<32>,
        flash_loan: Address,
        flash_loan_bytes: BytesN<32>,
    );

    fn deposit(e: Env, admin: Address, from: Address, amount: i128) -> i128;

    fn fee_withd(e: Env, admin: Address, to: Address, batch_ts: i128, shares: i128);

    fn get_shares(e: Env, id: Address, batch_ts: i128) -> Option<BatchObj>;

    fn batches(e: Env, id: Address) -> Vec<i128>;

    fn withdraw(e: Env, admin: Address, to: Address) -> i128;
}

pub struct VaultContract;

#[contractimpl]
impl VaultContractTrait for VaultContract {
    fn initialize(
        e: Env,
        admin: Address,
        token_id: BytesN<32>,
        flash_loan: Address,
        flash_loan_bytes: BytesN<32>,
    ) {
        log!(&e, "initializing");

        if has_administrator(&e) {
            panic!("admin is already set");
        }

        write_administrator(&e, admin);
        put_flash_loan(&e, flash_loan);
        put_flash_loan_bytes(&e, flash_loan_bytes);
        put_token_id(&e, token_id);
    }

    fn deposit(e: Env, admin: Address, from: Address, amount: i128) -> i128 {
        if read_admin(&e) != admin {
            panic!("not the admin")
        }
        admin.require_auth();

        transfer_in_vault(&e, &from, &amount);

        let contract_id = get_token_id(&e);
        let token_client = token::Client::new(&e, &contract_id);

        token_client.xfer(&get_contract_addr(&e), &get_flash_loan(&e), &amount);

        let tot_supply = get_tot_supply(&e);

        let shares = if 0 == tot_supply {
            amount
        } else {
            (amount * tot_supply) / (get_token_balance(&e) - amount)
        };

        e.storage().set(&DataKey::InitialDep(from.clone()), &amount);
        mint_shares(&e, from, shares, amount)
    }

    fn get_shares(e: Env, id: Address, batch_n: i128) -> Option<BatchObj> {
        let key = DataKey::Batch(BatchKey(id, batch_n));

        let batch: Option<BatchObj> = e.storage().get(&key).unwrap_or(Ok(None)).unwrap();

        batch
    }

    // Batches returns an integer `current_n`. Batches are stored with key `BatchKey(Address, current_n)`, so having `current_n` and iterating up to it (0..n) will help to gather all of the user's batches (you'll still need to filter for batches that have been completely withdrawn, thus deleted).
    fn batches(e: Env, id: Address) -> Vec<i128> {
        get_user_batches(&e, id)
    }

    fn fee_withd(e: Env, admin: Address, to: Address, batch_n: i128, shares: i128) {
        if read_admin(&e) != admin {
            panic!("not the admin")
        }

        admin.require_auth();

        let tot_supply = get_tot_supply(&e);
        let tot_bal = get_token_balance(&e);
        let batch: BatchObj = e
            .storage()
            .get(&DataKey::Batch(BatchKey(to.clone(), batch_n)))
            .unwrap()
            .unwrap();
        let deposit = batch.deposit;
        let init_s = batch.init_s;
        let curr_s = batch.curr_s;

        if curr_s < shares {
            panic!("not enough shares");
        }

        let new_deposit = deposit * (shares * 10000000 / init_s) / 10000000;

        let fee_amount = ((tot_bal * shares) / tot_supply) - new_deposit;
        if fee_amount >= 0 {
            transfer(&e, &to, fee_amount);
            burn_shares(&e, to.clone(), shares, batch_n);
            let new_tot_supply = get_tot_supply(&e);
            let new_tot_bal = get_token_balance(&e);

            if tot_bal != new_deposit {
                let new_shares = (new_deposit * new_tot_supply) / (new_tot_bal - new_deposit);
                mint_shares(&e, to, new_shares, new_deposit);
            } else {
                let new_shares = (new_deposit * tot_supply) / new_deposit;
                mint_shares(&e, to, new_shares, new_deposit);
            }
        }
    }

    fn withdraw(e: Env, admin: Address, to: Address) -> i128 {
        if read_admin(&e) != admin {
            panic!("not the admin")
        }

        admin.require_auth();

        let batches = get_user_batches(&e, to.clone());
        log!(&e, "batches {}", batches);

        let mut amount: i128 = 0;
        let mut temp_supply: i128 = get_tot_supply(&e);
        let mut temp_balance: i128 = get_token_balance(&e);

        for batch_el in batches.iter() {
            let batch_n = batch_el.unwrap_or_else(|_| panic!("no ts in batch"));

            let key = DataKey::Batch(BatchKey(to.clone(), batch_n));
            let batch: BatchObj = e
                .storage()
                .get(&key.clone())
                .unwrap_or_else(|| panic!("no batch with this id"))
                .unwrap();

            let deposit = batch.deposit;
            let init_s = batch.init_s;
            let curr_s = batch.curr_s;

            let new_deposit = deposit * (curr_s * 10000000 / init_s) / 10000000;
            let fee_amount = ((temp_balance * curr_s) / temp_supply) - new_deposit;

            amount += fee_amount;

            temp_balance -= fee_amount;
            temp_supply -= curr_s;

            burn_shares(&e, to.clone(), curr_s, batch_n);

            if temp_balance != new_deposit {
                temp_supply += (new_deposit * temp_supply) / (temp_balance - new_deposit);
                log!(&e, "deposit != balance", amount);
            } else {
                temp_supply += (new_deposit * temp_supply) / (new_deposit);
                log!(&e, "deposit == balance", amount);
            }
        }

        let initial_deposit = e
            .storage()
            .get::<DataKey, i128>(&DataKey::InitialDep(to.clone()))
            .unwrap()
            .unwrap();

        let fl_bytes_id = get_flash_loan_bytes(&e);
        let fl_client = flash_loan::Client::new(&e, &fl_bytes_id);
        fl_client.withdraw(&get_contract_addr(&e), &initial_deposit, &to);
        transfer(&e, &to, amount);
        amount
    }
}
