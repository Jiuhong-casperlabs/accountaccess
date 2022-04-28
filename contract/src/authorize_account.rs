#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use casper_contract::contract_api::runtime;

use casper_types::{runtime_args, ContractHash, RuntimeArgs, URef};

#[no_mangle]
pub extern "C" fn call() {
    let contract_hash_str: String = runtime::get_named_arg("contract_hash_str");
    let contract_hash = ContractHash::from_formatted_str(&contract_hash_str).unwrap();
    let a: URef = runtime::call_contract(contract_hash, "get_uref", runtime_args! {});

    runtime::put_key("access_uref", a.into());
}
