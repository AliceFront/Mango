// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::account_address::AccountAddress;
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentVASP {
    num_children: u64,
}

impl ParentVASP {
    pub fn num_children(&self) -> u64 {
        self.num_children
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ChildVASP {
    parent_vasp_addr: AccountAddress,
}

impl ChildVASP {
    pub fn parent_vasp_addr(&self) -> AccountAddress {
        self.parent_vasp_addr
    }
}

impl MoveStructType for ParentVASP {
    const MODULE_NAME: &'static IdentStr = ident_str!("VASP");
    const STRUCT_NAME: &'static IdentStr = ident_str!("ParentVASP");
}

impl MoveStructType for ChildVASP {
    const MODULE_NAME: &'static IdentStr = ident_str!("VASP");
    const STRUCT_NAME: &'static IdentStr = ident_str!("ChildVASP");
}

impl MoveResource for ParentVASP {}
impl MoveResource for ChildVASP {}
