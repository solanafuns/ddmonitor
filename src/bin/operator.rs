use {
    clap::Parser,
    contract::instruction::InstructionData,
    ddmonitor::{handlers, runtime, sdk},
    env_logger::Env,
    log::{error, info},
    solana_program::{instruction::AccountMeta, pubkey::Pubkey, system_program},
    solana_sdk::{instruction::Instruction, signer::Signer, transaction::Transaction},
};

/// One operator to push message to ddmonitor queue
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
    #[arg(long, default_value_t = String::from("local"))]
    network: String,

    /// Solana program address
    #[arg(short, long, default_value_t = String::from("HZRahcg3oLXw4GScUN7bzCfHWx33G6SBrg6G1vVL1qEm"))]
    program: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("ddmonitor operator start ...");

    let args = Args::parse();
    runtime::init_app();
    let network = sdk::Network::from_string(&args.network);
    info!("network is : <{:?}> ", network);
    let pair = sdk::init_solana_wallet()?;
    let pub_key = pair.pubkey();
    let connection = sdk::get_rpc_client(&network);
    let program_account = runtime::program_account(args.program.clone());

    info!("current wallet address : {}", pub_key);
    sdk::connection_available(&connection)?;

    sdk::confirm_balance(&connection, &network, &pub_key, 5);

    if !sdk::program_available(&connection, &program_account) {
        error!("program account is not available , exit...");
        return Ok(());
    }

    let queue_name = args.name.clone();
    let queue_pub = sdk::pda_queue_account(&program_account, &queue_name);

    let queue_avaliable = {
        let queue_info = connection.get_account(&queue_pub);
        if queue_info.is_err() {
            false
        } else {
            let queue_info = queue_info.unwrap();
            queue_info.owner == args.program.parse().unwrap()
                && queue_info.lamports > 0
                && !queue_info.executable
                && queue_info.data.len() > 0
        }
    };

    if !queue_avaliable {
        error!("queue {}<{}> is not available ", queue_pub, queue_name);
        return Ok(());
    }

    fn create_instruction(
        payer_pub: Pubkey,
        queue_pub: Pubkey,
        program_id: String,
        name: String,
        data: Vec<u8>,
    ) -> Instruction {
        info!(
            "you will push message : {:?} to : {}, queue account : {}",
            data,
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

    {
        let instruction = create_instruction(
            pub_key.clone(),
            queue_pub.clone(),
            args.program.clone(),
            args.name.clone(),
            args.message.clone().into_bytes(),
        );

        let blockhash = connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

        match connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                info!("send message tx : {:?}", tx);
            }
            Err(e) => {
                error!("send message error : {:?}", e);
            }
        }
    }

    {
        let instruction = create_instruction(
            pub_key.clone(),
            queue_pub.clone(),
            args.program.clone(),
            args.name.clone(),
            handlers::ActionInfo::ActionSample(1, 2).wrapper(),
        );

        let blockhash = connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

        match connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                info!("send message tx : {:?}", tx);
            }
            Err(e) => {
                error!("send message error : {:?}", e);
            }
        }
    }

    Ok(())
}
