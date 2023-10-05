use {
    crate::{models::ServerPrivate, runtime},
    base64::{
        alphabet,
        engine::{self, general_purpose},
        Engine as _,
    },
    borsh::{self, BorshDeserialize, BorshSerialize},
    solana_account_decoder::{UiAccountData, UiAccountEncoding},
    solana_client::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    solana_program::pubkey::Pubkey,
    solana_rpc_client_api::config::RpcAccountInfoConfig,
    solana_sdk::{commitment_config::CommitmentConfig, signer::keypair::Keypair},
    std::io::Result,
};

const PRIVATE_PATH: &str = "./private/private.json";

pub fn hello(msg: &str) {
    println!("hello world {} !", msg);
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    RegisterQueue { name: String },
    PushMessage { name: String },
    Empty(u8),
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

pub fn get_rpc_client() -> RpcClient {
    RpcClient::new_with_commitment(runtime::RPC_URL.to_string(), CommitmentConfig::finalized())
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

pub fn get_account_updates(account_pubkey: &Pubkey, callback: fn(String)) -> Result<()> {
    let (mut _account_subscription_client, account_subscription_receiver) =
        PubsubClient::account_subscribe(
            runtime::WS_URL,
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
