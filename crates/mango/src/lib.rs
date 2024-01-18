// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod account;
pub mod client_proxy;
pub mod common;
pub mod config;
pub mod genesis;
pub mod key;
pub mod mango_client;
pub mod move_tool;
pub mod node;

use clap::Parser;

// Command Line Interface (CLI) for interacting with the mango blockchain
#[derive(Parser)]
#[clap(name = "mango", author, version, propagate_version = true)]
pub enum Tool {
    #[clap(subcommand)]
    Account(account::AccountSubcommand),
    #[clap(subcommand)]
    Config(config::ConfigTool),
}

impl Tool {
    pub async fn execute(self) -> Result<String, String> {
        use Tool::*;
        match self {
            Account(tool) => tool.execute().await,
            Config(tool) => tool.execute().await,
        }
    }
}
