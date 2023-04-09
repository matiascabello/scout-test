mod token {
    use soroban_sdk::contractimport;

    contractimport!(file = "../soroban_token_spec.wasm");
}

mod vault {
    use soroban_sdk::contractimport;

    contractimport!(file = "../target/wasm32-unknown-unknown/release/xycloans_fl_vault.wasm");
}

mod loan_ctr {
    use soroban_sdk::contractimport;

    contractimport!(file = "../target/wasm32-unknown-unknown/release/xycloans_flash_loan.wasm");
}

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

#[test]
fn withdraw_liquidity_position() {
    let e: Env = Default::default();
    let admin1 = Address::random(&e);

    let user1 = Address::random(&e);
    let user2 = Address::random(&e);

    let token_id = e.register_stellar_asset_contract(admin1);
    let token = token::Client::new(&e, &token_id);

    let vault_contract_id =
        e.register_contract_wasm(&BytesN::from_array(&e, &[5; 32]), vault::WASM);
    let vault_client = vault::Client::new(&e, &vault_contract_id);
    let vault_id = Address::from_contract_id(&e, &vault_contract_id);

    let flash_loan_contract_id =
        e.register_contract_wasm(&BytesN::from_array(&e, &[8; 32]), loan_ctr::WASM);
    let flash_loan_id = Address::from_contract_id(&e, &flash_loan_contract_id);
    let flash_loan_client = loan_ctr::Client::new(&e, &flash_loan_contract_id);

    flash_loan_client.init(&token_id, &vault_id);
    vault_client.initialize(&user1, &token_id, &flash_loan_id, &flash_loan_contract_id);

    token.mint(&user1, &100000000000);
    token.mint(&user2, &100000000000);

    vault_client.deposit(&user1, &user1, &50000000000);

    assert_eq!(token.balance(&user1), 50000000000);

    vault_client.deposit(&user1, &user2, &100000000000);
    assert_eq!(token.balance(&user2), 0);

    vault_client.withdraw_fee(&user1, &user2, &0, &100000000000);
    assert_eq!(token.balance(&user2), 0);

    // fees arrive
    token.mint(&vault_id, &10000);

    vault_client.withdraw_fee(&user1, &user2, &1, &100000000000);
    assert_eq!(token.balance(&user2), 6666);

    vault_client.withdraw(&user1, &user1);
    assert_eq!(token.balance(&user1), 100000003334);
}