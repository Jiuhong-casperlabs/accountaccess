#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, ApiError, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, PublicKey, RuntimeArgs, URef,
};

pub const GROUP_LABEL: &str = "group_label";
pub const GROUP_UREF_NAME: &str = "group_uref";

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

#[no_mangle]
fn retrieve_urefs() {
    let urefs: Vec<URef> = get_key(&runtime::get_caller().to_string());
    if urefs.is_empty() {
        revert(ApiError::User(1));
    }
    runtime::ret(CLValue::from_t(urefs).unwrap_or_revert())
}

#[no_mangle]
fn append_urefs() {
    let urefs: Vec<URef> = runtime::get_named_arg("urefs");
    let mut users: Vec<PublicKey> = runtime::get_named_arg("account_pubkeys");
    if urefs.len() != users.len() {
        revert(ApiError::User(3));
    }

    for uref in urefs {
        let user_key = users.pop().unwrap_or_revert().to_account_hash().to_string();
        let mut personal_uref_list: Vec<URef> = get_key(&user_key);
        personal_uref_list.push(uref);
        set_key(&user_key, personal_uref_list);
    }
}

#[no_mangle]
fn group_access_only() {
    // JACKPOT revert with User error 777 to see without a doubt that we have access to this function.
    revert(ApiError::User(777))
}

#[no_mangle]
pub extern "C" fn call() {
    let account_pubkeys: Vec<PublicKey> = runtime::get_named_arg("account_pubkeys");

    let named_keys: BTreeMap<String, Key> = BTreeMap::new();

    let mut entry_points = EntryPoints::new();

    let entry_point_1 = EntryPoint::new(
        "append_urefs",
        vec![
            Parameter::new("urefs".to_string(), CLType::List(Box::new(CLType::URef))),
            Parameter::new(
                "account_pubkeys".to_string(),
                CLType::List(Box::new(CLType::PublicKey)),
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new(GROUP_LABEL)]),
        EntryPointType::Contract,
    );

    let entry_point_2 = EntryPoint::new(
        String::from("retrieve_urefs"),
        vec![
            Parameter::new("urefs".to_string(), CLType::List(Box::new(CLType::URef))),
            Parameter::new(
                "account_pubkeys".to_string(),
                CLType::List(Box::new(CLType::PublicKey)),
            ),
        ],
        CLType::List(Box::new(CLType::URef)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let entry_point_3 = EntryPoint::new(
        "group_access_only",
        vec![],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new(GROUP_LABEL)]),
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entry_point_1);
    entry_points.add_entry_point(entry_point_2);
    entry_points.add_entry_point(entry_point_3);

    // access - contract
    let (contract_package_hash, _access_uref) = storage::create_contract_package_at_hash();

    let mut admin_group = storage::create_contract_user_group(
        contract_package_hash,
        GROUP_LABEL,
        (account_pubkeys.len() + 1) as u8,
        Default::default(),
    )
    .unwrap();

    runtime::put_key(
        GROUP_UREF_NAME,
        casper_types::Key::URef(admin_group.pop().unwrap_or_revert()),
    );

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    let _: () = runtime::call_contract(
        contract_hash,
        "append_urefs",
        runtime_args! {"urefs" => admin_group, "account_pubkeys" => account_pubkeys},
    );
    runtime::put_key("account_access_contract", contract_hash.into());
    runtime::put_key(
        "account_access_contract_package",
        contract_package_hash.into(),
    );
}
