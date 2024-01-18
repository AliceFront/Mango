// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress, account_config::constants::CORE_ACCOUNT_MODULE_IDENTIFIER,
};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct KeyRotationCapabilityResource {
    account_address: AccountAddress,
}

impl MoveStructType for KeyRotationCapabilityResource {
    const MODULE_NAME: &'static IdentStr = CORE_ACCOUNT_MODULE_IDENTIFIER;
    const STRUCT_NAME: &'static IdentStr = ident_str!("KeyRotationCapability");
}

impl MoveResource for KeyRotationCapabilityResource {}
