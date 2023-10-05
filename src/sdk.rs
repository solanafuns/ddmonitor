use {
    crate::{models::ServerPrivate, runtime},
    borsh::{self, BorshDeserialize, BorshSerialize},
    solana_client::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::{commitment_config::CommitmentConfig, signer::keypair::Keypair},
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
