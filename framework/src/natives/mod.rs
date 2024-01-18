// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod account;
pub mod signature;
use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use move_vm_runtime::native_functions::{NativeFunction, NativeFunctionTable};
use std::sync::Arc;

pub fn all_natives(diem_framework_addr: AccountAddress) -> NativeFunctionTable {
    let natives: &[(&str, &str, NativeFunction)] = &[
        (
            "DiemAccount",
            "create_signer",
            Arc::new(account::native_create_signer),
        ),
        (
            "DiemAccount",
            "destroy_signer",
            Arc::new(account::native_destroy_signer),
        ),
        (
            "Signature",
            "ed25519_validate_pubkey",
            Arc::new(signature::native_ed25519_publickey_validation),
        ),
        (
            "Signature",
            "ed25519_verify",
            Arc::new(signature::native_ed25519_signature_verification),
        ),
    ];
    natives
        .iter()
        .cloned()
        .map(|(module_name, func_name, func)| {
            (
                diem_framework_addr,
                Identifier::new(module_name).unwrap(),
                Identifier::new(func_name).unwrap(),
                func,
            )
        })
        .collect()
}
