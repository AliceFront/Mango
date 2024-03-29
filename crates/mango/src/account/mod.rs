// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::common::types::Command;
use clap::Subcommand;

pub mod create_account;
pub mod mint;

/// Tool to interact with account data
///
/// This is used to create, request information and facilitate transfers between accounts.
#[derive(Debug, Subcommand)]
pub enum AccountSubcommand {
    Create(create_account::CreateAccount),
    Mint(mint::MintAccount),
}

impl AccountSubcommand {
    pub async fn execute(self) -> Result<String, String> {
        match self {
            AccountSubcommand::Create(tool) => tool.execute_serialized().await,
            AccountSubcommand::Mint(tool) => tool.execute_serialized().await,
        }
    }
}
