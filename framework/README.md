---
id: mango-framework
title: mango Framework
custom_edit_url: https://github.com/mango/mango/edit/main/language/mango-framework/README.md
---

## The mango Framework

The mango Framework defines the standard actions that can be performed on-chain
both by the mango VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the mango Framework and contributing guidelines are documented separately. Particularly:
* Documentation for the set of allowed transaction script can be found in [script_documentation/script_documentation.md](script_documentation/script_documentation.md).
* The overview documentation for the Move modules can be found in [modules/doc/overview.md](modules/doc/overview.md).
* An overview of the approach to formal verification of the framework can be found in [script_documentation/spec_documentation.md](script_documentation/spec_documentation.md).
* Contributing guidelines and basic coding standards for the mango Framework can be found in [CONTRIBUTING.md](CONTRIBUTING.md).

## Compilation and Generation

Recompilation of the mango Framework and the regeneration of the documents,
ABIs, and error information can be performed by running `cargo run` from this
directory. There are a number of different options available and these are
explained in the help for this command by running `cargo run -- --help` in this
directory. Compilation and generation will be much faster if run in release
mode (`cargo run --release`).

## Layout
The overall structure of the mango Framework is as follows:

```
├── compiled                                # Generated files and public rust interface to the mango Framework
│   ├── error_descriptions/*.errmap         # Generated error descriptions for use by the Move Explain tool
│   ├── src                                 # External Rust interface/library to use the mango Framework
│   ├── stdlib                              # The compiled Move bytecode of the mango Framework source modules
│   ├── script_abis                         # Generated ABIs for script function transactions, and all new transactions
│   └── legacy/transaction_scripts          # Legacy generated ABIs and bytecode for each transaction script in the allowlist
│       ├── abi/*.abi                       # Directory containing generated ABIs for legacy transaction scripts
│       └── *.mv
├── modules                                 # mango Framework source modules, script modules, and generated documentation
│   ├── *.move
│   └── doc/*.md                            # Generated documentation for the mango Framework modules
├── nursery/*.move                          # Move modules that are not published on-chain, but are used for testing and debugging locally
├── src                                     # Compilation and generation of information from Move source files in the mango Framework. Not designed to be used as a Rust library
├── tests
└── script_documentation/*.md               # Generated documentation for allowed transaction scripts
```
