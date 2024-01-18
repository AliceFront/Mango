// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use diem_vm::DiemVM;
use diemdb::DiemDB;
use executor::db_bootstrapper;
use mango_config::config::RocksdbConfig;
use mango_management::{config::ConfigPath, error::Error, secure_backend::SharedBackend};
use mango_temppath::TempPath;
use mango_types::{chain_id::ChainId, transaction::Transaction, waypoint::Waypoint};
use storage_interface::DbReaderWriter;
use structopt::StructOpt;

/// Produces a waypoint from Genesis from the shared storage. It then computes the Waypoint and
/// optionally inserts it into another storage, typically the validator storage.
#[derive(Debug, StructOpt)]
pub struct CreateWaypoint {
    #[structopt(flatten)]
    config: ConfigPath,
    #[structopt(long, required_unless("config"))]
    chain_id: Option<ChainId>,
    #[structopt(flatten)]
    shared_backend: SharedBackend,
}

impl CreateWaypoint {
    pub fn execute(self) -> Result<Waypoint, Error> {
        let genesis_helper = crate::genesis::Genesis {
            config: self.config,
            chain_id: self.chain_id,
            backend: self.shared_backend,
            path: None,
        };

        let genesis = genesis_helper.execute()?;

        create_genesis_waypoint(&genesis)
    }
}

pub fn create_genesis_waypoint(genesis: &Transaction) -> Result<Waypoint, Error> {
    let path = TempPath::new();
    let diemdb = DiemDB::open(&path, false, None, RocksdbConfig::default())
        .map_err(|e| Error::UnexpectedError(e.to_string()))?;
    let db_rw = DbReaderWriter::new(diemdb);

    db_bootstrapper::generate_waypoint::<DiemVM>(&db_rw, genesis)
        .map_err(|e| Error::UnexpectedError(e.to_string()))
}
