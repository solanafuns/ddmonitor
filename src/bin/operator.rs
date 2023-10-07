use {
    clap::Parser,
    contract::instruction::InstructionData,
    ddmonitor::{runtime, sdk},
    solana_program::{instruction::AccountMeta, system_program},
    solana_sdk::{instruction::Instruction, signer::Signer, transaction::Transaction},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the queue
    #[arg(short, long, default_value_t = String::from("default"))]
    name: String,

    /// Message to push
    #[arg(short, long, default_value_t = String::from("hello world"))]
    message: String,

    /// Network to communicate with
    #[arg(short, long, default_value_t = String::from("local"))]
    network: String,

    /// Solana program address
    #[arg(short, long, default_value_t = String::from("HZRahcg3oLXw4GScUN7bzCfHWx33G6SBrg6G1vVL1qEm"))]
    program: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let network = sdk::Network::from_string(&args.network);
    runtime::init_app();
    let pair = sdk::init_solana_wallet()?;
    let pub_key = pair.pubkey();
    println!("current wallet address : {}", pub_key);

    let connection = sdk::get_rpc_client(&network);
    sdk::confirm_balance(&connection, &network, &pub_key, 5);
    let queue_name = args.name.clone();
    let queue_account = sdk::pda_queue_account(&network, &queue_name);

    println!(
        "you will push message : {} to : {}, queue account : {}",
        args.message,
        args.name,
        queue_account.to_string()
    );

    let accounts = vec![
        AccountMeta::new(pub_key.clone(), true),
        AccountMeta::new(queue_account.clone(), false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction::new_with_borsh(
        runtime::program_account(network.program_address()),
        &InstructionData::PushMessage {
            name: args.name,
            data: args.message,
        },
        accounts,
    );

    let blockhash = connection.get_latest_blockhash().unwrap();
    let transaction =
        Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

    match connection.send_and_confirm_transaction(&transaction) {
        Ok(tx) => {
            println!("send message tx : {:?}", tx);
        }
        Err(e) => {
            println!("send message error : {:?}", e);
        }
    }
    Ok(())
}
