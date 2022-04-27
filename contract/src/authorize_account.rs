#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use casper_contract::contract_api::runtime::{self, revert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef};
#[no_mangle]
fn call() {
    let this_contract_package: ContractPackageHash = runtime::get_named_arg("this_contract");
    let urefs: Vec<URef> = runtime::call_versioned_contract(
        this_contract_package,
        None,
        "retrieve_urefs",
        runtime_args! {},
    );
    if urefs.is_empty() {
        revert(ApiError::User(2));
    }
    for uref in urefs {
        if uref == URef::default() {
            revert(ApiError::User(4))
        }
        runtime::put_key(&uref.to_string(), Key::URef(uref));
    }
}
