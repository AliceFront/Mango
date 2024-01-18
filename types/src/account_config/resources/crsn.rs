// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AccountSequenceInfo {
    Sequential(u64),
    CRSN { min_nonce: u64, size: u64 },
}

impl AccountSequenceInfo {
    pub fn min_seq(&self) -> u64 {
        match self {
            Self::Sequential(seqno) => *seqno,
            Self::CRSN { min_nonce, .. } => *min_nonce,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct CRSNResource {
    min_nonce: u64,
    size: u64,
    // NG: The length of these slots are not necessarily the size of the CRSN.
    slots: Vec<u8>,
}

impl CRSNResource {
    pub fn min_nonce(&self) -> u64 {
        self.min_nonce
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl MoveStructType for CRSNResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("CRSN");
    const STRUCT_NAME: &'static IdentStr = ident_str!("CRSN");
}

impl MoveResource for CRSNResource {}
