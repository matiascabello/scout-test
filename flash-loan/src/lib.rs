#![no_std]
//mod token {
//    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
//}

mod vault {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/xycloans_vault_interface.wasm"
    );
}

mod contract;
mod execution;
mod storage;
mod token_utility;
mod types;
