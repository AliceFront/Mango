// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{on_chain_config::OnChainConfig, validator_info::ValidatorInfo};

use serde::{Deserialize, Serialize};
use std::{fmt, iter::IntoIterator, vec};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum ConsensusScheme {
    Ed25519 = 0,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]

pub struct ValidatorSet {
    scheme: ConsensusScheme,
    payload: Vec<ValidatorInfo>,
}

impl fmt::Display for ValidatorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for validator in self.payload().iter() {
            write!(f, "{} ", validator)?;
        }
        write!(f, "]")
    }
}

impl ValidatorSet {
    /// Constructs a ValidatorSet resource.
    pub fn new(payload: Vec<ValidatorInfo>) -> Self {
        Self {
            scheme: ConsensusScheme::Ed25519,
            payload,
        }
    }

    pub fn payload(&self) -> &[ValidatorInfo] {
        &self.payload
    }

    pub fn empty() -> Self {
        ValidatorSet::new(Vec::new())
    }
}

impl OnChainConfig for ValidatorSet {
    // validator_set_address
    const IDENTIFIER: &'static str = "DiemSystem";
}

impl IntoIterator for ValidatorSet {
    type Item = ValidatorInfo;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.payload.into_iter()
    }
}
