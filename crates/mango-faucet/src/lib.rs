// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

//! This crates provides Faucet service for creating Testnet with simplified on-chain account creation
//! and minting coins.
//!
//! THIS SERVICE SHOULD NEVER BE DEPLOYED TO MAINNET.
//!
//! ## Launch service
//!
//! Launch faucet service local and connect to Testnet:
//!
//! ```bash
//! cargo run -p mango-faucet -- -a 127.0.0.1 -p 8080 -c TESTNET \
//!     -m <treasury-compliance-private-key-path> -s https://testnet.mango.com/v1
//! ```
//!
//! Check help doc for options details:
//!
//! ```bash
//! cargo run -p mango-faucet -- -h
//! ```
//!
//! ## Development
//!
//! Test with mango-swarm by add -m option:
//!
//! ```bash
//! cargo run -p mango-swarm -- -s -l -n 1 -m
//! ```
//!

pub mod mint;
