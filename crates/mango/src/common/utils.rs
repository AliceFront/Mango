// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0
use crate::{
    client_proxy::{retrieve_waypoint, ClientProxy},
    common::{config::ConfigPath, types::CliError},
};
use anyhow::Result;
use mango_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    test_utils::KeyPair,
    Uniform, ValidCryptoMaterialStringExt,
};
use mango_types::{chain_id::ChainId, transaction::authenticator::AuthenticationKey};

use rand::{prelude::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs,
    fs::File,
    io::{self, Write},
    str::{self, FromStr},
};

use swiss_knife::helpers;

use mango_types::account_address::AccountAddress;

use home;

#[cfg(unix)]

/// Convert result to JSON output
pub async fn to_common_result<T: Serialize>(result: Result<T, CliError>) -> Result<String, String> {
    let is_err = result.is_err();
    let result: ResultWrapper<T> = result.into();
    let string = serde_json::to_string_pretty(&result).unwrap();
    if is_err {
        Err(string)
    } else {
        Ok(string)
    }
}

#[derive(Debug, Serialize)]
enum ResultWrapper<T> {
    Result(T),
    Error(String),
}

impl<T> From<Result<T, CliError>> for ResultWrapper<T> {
    fn from(result: Result<T, CliError>) -> Self {
        match result {
            Ok(inner) => ResultWrapper::Result(inner),
            Err(inner) => ResultWrapper::Error(inner.to_string()),
        }
    }
}

/// Converts a GenerateKeypairResponse struct to JSON and saves file
pub fn save_keypair(keypair: GenerateKeypairResponse) -> Result<String, Box<dyn Error>> {
    let serialized = serde_json::to_string_pretty(&keypair).unwrap();
    println!("key_pair: {}", serialized);

    let folder = match home::home_dir() {
        Some(path) => path.display().to_string() + "/.mango/account",
        None => env::current_dir()?.display().to_string(),
    };

    if !std::path::Path::new(&folder).exists() {
        match fs::create_dir_all(&folder) {
            Ok(()) => {}
            Err(err) => {
                panic!("Error creating directory to save keypair file: {}", err)
            }
        };
    }
    let file_path = format!("{}/{}-keypair.json", &folder, &keypair.diem_account_address);

    let file = match File::create(&file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.into()),
    };

    match serde_json::to_writer_pretty(&file, &keypair) {
        Ok(()) => {
            let res = format!("Keypair successfully saved to {}", &file_path);
            Ok(res)
        }
        Err(err) => Err(err.into()),
    }
}

/// Response struct for generating a new keypair
/// Moved from swiss knife
#[derive(Deserialize, Serialize, Clone)]
pub struct GenerateKeypairResponse {
    pub private_key: String,
    pub public_key: String,
    pub diem_auth_key: String,
    pub diem_account_address: String,
}

/// Generates a new local public/private keypair
/// Returns a GenerateKeypairResponse struct
pub fn generate_key_pair(seed: Option<u64>) -> GenerateKeypairResponse {
    let mut rng = StdRng::seed_from_u64(seed.unwrap_or_else(rand::random));
    let keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey> =
        Ed25519PrivateKey::generate(&mut rng).into();

    let diem_auth_key = AuthenticationKey::ed25519(&keypair.public_key);
    let diem_account_address: String = diem_auth_key.derived_address().to_string();
    let diem_auth_key: String = diem_auth_key.to_string();
    GenerateKeypairResponse {
        private_key: keypair
            .private_key
            .to_encoded_string()
            .map_err(|err| {
                helpers::exit_with_error(format!("Failed to encode private key : {}", err))
            })
            .unwrap(),
        public_key: keypair
            .public_key
            .to_encoded_string()
            .map_err(|err| {
                helpers::exit_with_error(format!("Failed to encode public key : {}", err))
            })
            .unwrap(),
        diem_auth_key,
        diem_account_address,
    }
}

/// Struct used to store data for each created account.  We track the sequence number
/// so we can create new transactions easily
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(Clone))]
pub struct AccountData {
    /// Address of the account.
    pub address: AccountAddress,
    /// Authentication key of the account.
    pub authentication_key: Option<Vec<u8>>,
    /// (private_key, public_key) pair if the account is not managed by wallet.
    pub key_pair: Option<KeyPair<Ed25519PrivateKey, Ed25519PublicKey>>,
    /// Latest sequence number maintained by client, it can be different from validator.
    pub sequence_number: u64,
    /// Whether the account is initialized on chain, cached local only, or status unknown.
    pub status: AccountStatus,
}

/// Enum used to represent account status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    /// Account exists only in local cache, it is not persisted on chain.
    Local,
    /// Account is persisted on chain.
    Persisted,
    /// Not able to check account status, probably because client is not able to talk to the
    /// validator.
    Unknown,
}

// For commands where user additional input might be required
pub fn prompt_user(prompt: &'static str) -> Result<String, CliError> {
    print!("{}:", prompt);
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_line(&mut buffer)
        .map_err(|err| CliError::UserInputError(err.to_string()))?;
    Ok(buffer)
}

pub async fn mint_new_account(
    account: String,
    amount: u64,
    currency: String,
) -> Result<String, CliError> {
    let config_path = ConfigPath::default();
    let config = config_path.load().unwrap();

    let chain_str = String::from(&config.chain);
    let chain_id = ChainId::from_str(&chain_str).unwrap();
    let rpc = &config.rpc_endpoint;
    let faucet_url = config.faucet_endpoint.clone();
    let waypoint = retrieve_waypoint(&config.waypoint_url).await.unwrap();

    // mango root account/Faucet, TreasuryCompliance and DD use the same keypair for now
    let diem_root_account_file = "".to_string();
    let treasury_compliance_account_file = diem_root_account_file.clone();
    let dd_account_file = diem_root_account_file.clone();

    let mut client_proxy = ClientProxy::new(
        chain_id,
        rpc,
        &diem_root_account_file,
        &treasury_compliance_account_file,
        &dd_account_file,
        Some(faucet_url),
        waypoint,
        false,
    )
    .await
    .expect("Failed to construct client.");

    match client_proxy
        .mint_coins(account.clone(), amount, currency.clone())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            panic!("Error minting coins: {}", e)
        }
    };

    let amount = amount.to_string();
    let currency = currency;
    let account = account;
    let result = format!("Successfully minted {amount} {currency} to {account}");
    Ok(result)
}

// essentially runs account create and mint with default values
pub async fn create_new_default_account() -> Result<String, CliError> {
    // generate key pair locally first
    println!("Generating key pair locally");
    let keypair = generate_key_pair(None);
    if let Err(_err) = save_keypair(keypair.clone()) {
        panic!("Error saving keypair: {}", _err)
    };

    // create new account on network
    match mint_new_account(keypair.diem_auth_key, 1000, "XUS".to_string()).await {
        Ok(res) => Ok(res),
        Err(err) => panic!("Error saving keypair {}", err),
    }
}
