// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

// CLI for interacting with the mango blockchain

#![forbid(unsafe_code)]

use clap::Parser;
use mango::Tool;
use std::process::exit;

#[tokio::main]
async fn main() {
    let result = Tool::parse().execute().await;

    match result {
        Ok(res) => println!("{}", res),
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}
