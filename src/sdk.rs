use {
    crate::{runtime, runtime::ServerPrivate},
    base64::{
        alphabet,
        engine::{self, general_purpose},
        Engine as _,
    },
    solana_account_decoder::{UiAccountData, UiAccountEncoding},
    solana_client::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    solana_program::pubkey::Pubkey,
    solana_rpc_client_api::config::RpcAccountInfoConfig,
    solana_sdk::{commitment_config::CommitmentConfig, signer::keypair::Keypair},
    std::io::Result,
    std::{thread, time},
};

const PRIVATE_PATH: &str = "./private/private.json";

pub fn hello(msg: &str) {
    println!("hello world {} !", msg);
}

pub fn init_solana_wallet() -> std::io::Result<Keypair> {
    // open config file
    let config_path = runtime::app_path(PRIVATE_PATH);
    match std::fs::read_to_string(config_path) {
        Ok(config) => {
            let config = serde_json::from_str::<ServerPrivate>(&config).unwrap();
            Ok(Keypair::from_base58_string(&config.secret))
        }
        Err(e) => {
            println!("error reading config file : {}", e);
            let wallet = Keypair::new();
            let s = &ServerPrivate {
                secret: wallet.to_base58_string(),
            };
            let secret_config: String = serde_json::to_string(s).unwrap();
            std::fs::write(PRIVATE_PATH, secret_config)?;
            Ok(wallet)
        }
    }
}

pub fn get_rpc_client(network: &Network) -> RpcClient {
    RpcClient::new_with_commitment(network.get_rpc_url(), CommitmentConfig::finalized())
}

pub fn pda_queue_account(name: &str) -> Pubkey {
    let program_account = runtime::program_account();
    println!(
        "program_account is : {} , name is : {}",
        program_account.to_string(),
        name
    );
    let (pda, _nonce) = Pubkey::find_program_address(&[name.as_bytes()], &program_account);
    pda
}

pub fn get_account_updates(
    network: &Network,
    account_pubkey: &Pubkey,
    callback: fn(String),
) -> Result<()> {
    let (mut _account_subscription_client, account_subscription_receiver) =
        PubsubClient::account_subscribe(
            &network.get_ws_url(),
            account_pubkey,
            Some(RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::JsonParsed),
                data_slice: None,
                commitment: Some(CommitmentConfig::confirmed()),
                min_context_slot: None,
            }),
        )
        .unwrap();

    println!(
        "begin loop account event with : {}",
        account_pubkey.to_string()
    );
    loop {
        match account_subscription_receiver.recv() {
            Ok(response) => {
                println!("account subscription received");
                let ui_account = response.value;
                match ui_account.data {
                    UiAccountData::Binary(b64_str, _encoding) => {
                        callback(b64_str);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("account subscription error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn base64_decode(data_b64: &str) -> Result<Vec<u8>> {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    match engine.decode(data_b64) {
        Ok(data) => Ok(data),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("base64 decode error : {:?}", e),
        )),
    }
}

pub fn base64_encode(data: &[u8]) -> String {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    engine.encode(data)
}

pub fn confirm_balance(
    connection: &RpcClient,
    network: &Network,
    pub_key: &Pubkey,
    sol_count: u64,
) {
    loop {
        let balance = connection.get_balance(&pub_key).unwrap();
        println!("current balance is : {}", balance);
        if balance >= runtime::LAMPORTS_PER_SOL * sol_count {
            break;
        } else {
            if network.airdrop_enable() {
                let _ = connection.request_airdrop(&pub_key, runtime::LAMPORTS_PER_SOL * sol_count);
            }
            let delay = time::Duration::from_secs(3);
            thread::sleep(delay);
        }
    }
}

pub enum Network {
    Local,
    Dev,
    Test,
    MainBeta,
}

impl Network {
    pub fn from_string(network_name: &str) -> Self {
        match network_name {
            "local" => Self::Local,
            "dev" => Self::Dev,
            "test" => Self::Test,
            "main-beta" => Self::MainBeta,
            _ => Self::Local,
        }
    }

    pub fn get_ws_url(&self) -> String {
        match self {
            Self::Local => "ws://127.0.0.1:8900".to_string(),
            Self::Dev => "wss://api.devnet.solana.com/".to_string(),
            Self::Test => "wss://api.testnet.solana.com/".to_string(),
            Self::MainBeta => "wss://api.mainnet-beta.solana.com/".to_string(),
        }
    }

    pub fn get_rpc_url(&self) -> String {
        match self {
            Self::Local => "http://127.0.0.1:8899".to_string(),
            Self::Dev => "https://api.devnet.solana.com".to_string(),
            Self::Test => "https://api.testnet.solana.com".to_string(),
            Self::MainBeta => "https://api.mainnet-beta.solana.com".to_string(),
        }
    }

    pub fn airdrop_enable(&self) -> bool {
        match self {
            Self::Local => true,
            Self::Dev => true,
            Self::Test => true,
            Self::MainBeta => false,
        }
    }
}
