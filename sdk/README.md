# mango-sdk

[![mango-sdk on crates.io](https://img.shields.io/crates/v/mango-sdk)](https://crates.io/crates/mango-sdk)
[![Documentation (latest release)](https://docs.rs/mango-sdk/badge.svg)](https://docs.rs/mango-sdk/)
[![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mango.github.io/mango/diem_sdk/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/mango/mango/blob/main/LICENSE)

The official Rust SDK for mango.

## Usage

This SDK provides all the necessary components for building on top of the mango Blockchain. Some of the important modules are:

* `client` - Includes a [JSON-RPC client](https://github.com/mango/mango/blob/master/json-rpc/json-rpc-spec.md) implementation
* `crypto` - Types used for signing and verifying
* `transaction_builder` - Includes helpers for constructing transactions
* `types` - Includes types for mango on-chain data structures

## License

Mango Blockchain is licensed as [Apache 2.0](https://github.com/mango/mango/blob/main/LICENSE).
