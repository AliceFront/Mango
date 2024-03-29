// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use cli::client_proxy::ClientProxy;
use mango_client::BlockingClient;
use mango_config::config::NodeConfig;
use mango_temppath::TempPath;
use mango_types::{chain_id::ChainId, waypoint::Waypoint};
use nix::{
    sys::signal::{kill, SIGKILL},
    unistd::Pid,
};
use std::{fmt, fs::File, io::Write, path::PathBuf, process::Stdio, time::Duration};

#[derive(Debug)]
pub struct NodeInfo {
    pub chain_id: ChainId,
    pub json_rpc: String,
    pub root_key_path: PathBuf,
    pub waypoint: Waypoint,
    pub local_node_info: Option<LocalNodeInfo>,
}

#[derive(Debug)]
pub struct LocalNodeInfo {
    pub log_path: PathBuf,
    pub config_path: TempPath,
    node_pid: Pid,
}

fn diem_root_folder() -> PathBuf {
    let mut diem_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    diem_root_dir.pop();
    diem_root_dir.pop();
    diem_root_dir.pop();
    diem_root_dir
}

fn wait_till_healthy(json_rpc: &str) {
    let client = BlockingClient::new(json_rpc);
    loop {
        if client.get_metadata().is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

impl NodeInfo {
    pub fn new_local() -> Self {
        let config_temp_path = mango_temppath::TempPath::new();
        let config_path = config_temp_path.as_ref().to_path_buf();
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&config_path)
            .unwrap();
        let config_path = config_path.canonicalize().unwrap();

        // Build a single validator network
        let template = NodeConfig::default_for_validator();
        let builder = diem_genesis_tool::validator_builder::ValidatorBuilder::new(
            &config_path,
            diem_framework_releases::current_module_blobs().to_vec(),
        )
        .template(template)
        .randomize_first_validator_ports(true);
        let (root_keys, _genesis, genesis_waypoint, validators) =
            builder.build(rand::rngs::OsRng).unwrap();

        let diem_root_key_path = config_path.join("mint.key");
        let serialized_keys = bcs::to_bytes(&root_keys.root_key).unwrap();
        let mut key_file = std::fs::File::create(&diem_root_key_path).unwrap();
        key_file.write_all(&serialized_keys).unwrap();

        let mut log_file = config_path;
        log_file.push("validator.log");
        let log = File::create(log_file.as_path()).unwrap();

        let child_pid = std::process::Command::new("cargo")
            .args(&[
                "run",
                "--bin",
                "mango-node",
                "--",
                "--config",
                validators[0].config_path().to_str().unwrap(),
            ])
            .current_dir(diem_root_folder())
            .stderr(log)
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .id();

        let config = NodeConfig::load(validators[0].config_path()).unwrap();
        let json_rpc = format!("http://localhost:{}", config.json_rpc.address.port());

        wait_till_healthy(json_rpc.as_str());

        NodeInfo {
            chain_id: ChainId::test(),
            json_rpc,
            root_key_path: diem_root_key_path,
            waypoint: genesis_waypoint,
            local_node_info: Some(LocalNodeInfo {
                log_path: log_file,
                config_path: config_temp_path,
                node_pid: Pid::from_raw(child_pid as i32),
            }),
        }
    }

    pub fn get_client(&self) -> ClientProxy {
        let root_key = self.root_key_path.to_str().unwrap();
        ClientProxy::new(
            self.chain_id,
            self.json_rpc.as_str(),
            root_key,
            root_key,
            root_key,
            false,
            None,
            None,
            self.waypoint,
            false,
        )
        .expect("Failed to spawn a client from the NodeInfo")
    }
}

impl Drop for LocalNodeInfo {
    fn drop(&mut self) {
        kill(self.node_pid, Some(SIGKILL)).unwrap();
    }
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "json-rpc endpoint: {}", self.json_rpc)?;
        writeln!(f, "waypoint: {}", self.waypoint)?;
        writeln!(f, "chain-id: {}", self.chain_id)?;
        writeln!(f, "root-key path: {:?}", self.root_key_path)?;
        if let Some(info) = &self.local_node_info {
            writeln!(f, "config path: {:?}", info.config_path.path())?;
            writeln!(f, "log path: {:?}", info.log_path)?;
        }
        Ok(())
    }
}
