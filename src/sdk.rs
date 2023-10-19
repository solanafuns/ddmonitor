use {
    crate::{runtime, runtime::ServerPrivate},
    base64::{
        alphabet,
        engine::{self, general_purpose},
        Engine as _,
    },
    contract::instruction::InstructionData,
    log::{error, info},
    solana_account_decoder::{UiAccountData, UiAccountEncoding},
    solana_client::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    solana_program::{instruction::AccountMeta, pubkey::Pubkey, system_program},
    solana_rpc_client_api::config::RpcAccountInfoConfig,
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, signer::keypair::Keypair,
        signer::Signer, transaction::Transaction,
    },
    std::{io::Result, thread, time},
};

const PRIVATE_PATH: &str = "./private/private.json";

pub fn init_solana_wallet() -> std::io::Result<Keypair> {
    // open config file
    let config_path = runtime::app_path(PRIVATE_PATH);
    match std::fs::read_to_string(config_path) {
        Ok(config) => {
            let config = serde_json::from_str::<ServerPrivate>(&config).unwrap();
            Ok(Keypair::from_base58_string(&config.secret))
        }
        Err(e) => {
            error!("error reading config file : {}", e);
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
    RpcClient::new_with_commitment(network.get_rpc_url(), CommitmentConfig::confirmed())
}

pub fn pda_queue_account(program_account: &Pubkey, name: &str) -> Pubkey {
    info!(
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

    info!(
        "begin loop account event with : {}",
        account_pubkey.to_string()
    );
    loop {
        match account_subscription_receiver.recv() {
            Ok(response) => {
                info!("account subscription received");
                let ui_account = response.value;
                match ui_account.data {
                    UiAccountData::Binary(b64_str, _encoding) => {
                        callback(b64_str);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("account subscription error: {:?}", e);
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

pub fn program_available(connection: &RpcClient, program_id: &Pubkey) -> bool {
    match connection.get_account(program_id) {
        Ok(program_info) => {
            info!("program_info is : {:?}", program_info);
            program_info.lamports > 0 && program_info.executable && program_info.data.len() > 0
        }
        Err(e) => {
            error!("get program info error : {:?}", e);
            false
        }
    }
}

pub fn create_instruction(
    payer_pub: Pubkey,
    queue_pub: Pubkey,
    program_id: String,
    name: String,
    data: Vec<u8>,
) -> Instruction {
    info!(
        "you will push message with length {} to : {}, queue account : {}",
        data.len(),
        name,
        queue_pub.to_string()
    );
    let accounts = vec![
        AccountMeta::new(payer_pub, true),
        AccountMeta::new(queue_pub, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    Instruction::new_with_borsh(
        runtime::program_account(program_id),
        &InstructionData::PushMessage { name, data },
        accounts,
    )
}

pub fn send_instruction(
    connection: &RpcClient,
    payer: &Pubkey,
    singers: &Vec<&Keypair>,
    instruction: Instruction,
) {
    let blockhash = connection.get_latest_blockhash().unwrap();
    let transaction =
        Transaction::new_signed_with_payer(&[instruction], Some(&payer), singers, blockhash);

    match connection.send_and_confirm_transaction(&transaction) {
        Ok(tx) => {
            info!("send message tx : {:?}", tx);
        }
        Err(e) => {
            error!("send message error : {:?}", e);
        }
    }
}

pub fn connection_available(connection: &RpcClient) -> Result<bool> {
    match connection.get_version() {
        Ok(version) => {
            info!("solana version : {}", version.solana_core);
            Ok(true)
        }
        Err(e) => {
            error!("get solana version error : {:?}", e);
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("get solana version error : {:?}", e),
            ))
        }
    }
}

pub fn confirm_balance(
    connection: &RpcClient,
    network: &Network,
    pub_key: &Pubkey,
    air_drop_sol: u64,
) {
    loop {
        let balance = connection.get_balance(&pub_key).unwrap();
        info!("current balance is : {}", balance);
        if balance >= runtime::LAMPORTS_PER_SOL {
            break;
        } else {
            if network.airdrop_enable() {
                info!("airdrop sol : {}", air_drop_sol);
                let _ =
                    connection.request_airdrop(&pub_key, runtime::LAMPORTS_PER_SOL * air_drop_sol);
            }
            let delay = time::Duration::from_secs(3);
            thread::sleep(delay);
        }
    }
}

pub fn ddmonitor_init(
    network: &str,
    program: &str,
) -> Result<(Network, Keypair, Pubkey, RpcClient, Pubkey)> {
    let network: Network = Network::from_string(network);
    info!("network is : <{:?}> ", network);
    let pair = init_solana_wallet()?;
    let pub_key = pair.pubkey();
    let connection = get_rpc_client(&network);
    info!("current wallet address : {}", &pub_key);
    connection_available(&connection)?;
    confirm_balance(&connection, &network, &pub_key, 5);
    let program_account = runtime::program_account(program.to_string());
    if !program_available(&connection, &program_account) {
        error!("program account is not available , exit...");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("program account is not available , exit..."),
        ));
    }
    Ok((network, pair, pub_key, connection, program_account))
}

#[derive(Debug)]
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
