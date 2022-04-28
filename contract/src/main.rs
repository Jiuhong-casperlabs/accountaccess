#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::{string::ToString, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    contracts::NamedKeys, ApiError, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, PublicKey,
};

#[no_mangle]
pub extern "C" fn test_with_restriction() {}

#[no_mangle]
pub extern "C" fn test2() {}

#[no_mangle]
pub extern "C" fn get_uref() {
    let account_hash_str = runtime::get_caller().to_string();

    let account_hash_uref = match runtime::get_key(&account_hash_str) {
        Some(uref) => uref.into_uref().unwrap(),
        None => runtime::revert(ApiError::User(1)),
    };

    let return_value = CLValue::from_t(account_hash_uref).unwrap_or_revert();
    runtime::ret(return_value)
}

#[no_mangle]
pub extern "C" fn call() {
    let pks: Vec<PublicKey> = runtime::get_named_arg("pks");
    let (contract_package_hash, _access_uref) = storage::create_contract_package_at_hash();

    let mut admin_group = storage::create_contract_user_group(
        contract_package_hash,
        "my_group_label",
        (pks.len() + 1) as u8,
        Default::default(),
    )
    .unwrap();

    runtime::put_key("my_uref_name", admin_group.pop().unwrap().into());

    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "test_with_restriction",
        vec![],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("my_group_label")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "test2",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_uref",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = NamedKeys::new();

    for (i, uref) in admin_group.into_iter().enumerate() {
        named_keys.insert(pks[i].to_account_hash().to_string(), uref.into());
    }

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    runtime::put_key("accesscontract", contract_hash.into());
    runtime::put_key("accesscontractpackage", contract_package_hash.into());
}
