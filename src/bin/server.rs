use {
    clap::Parser,
    contract::{instruction::InstructionData, models},
    ddmonitor::{handlers, runtime, sdk},
    solana_program::{instruction::AccountMeta, system_program},
    solana_sdk::{instruction::Instruction, signer::Signer, transaction::Transaction},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the queue
    #[arg(short, long, default_value_t = String::from("default"))]
    name: String,

    #[arg(short, long, default_value_t = String::from("default"))]
    allow: String,

    /// Network to communicate with
    #[arg(short, long, default_value_t = String::from("local"))]
    network: String,

    /// Solana program address
    #[arg(short, long, default_value_t = String::from("HZRahcg3oLXw4GScUN7bzCfHWx33G6SBrg6G1vVL1qEm"))]
    program: String,
}

const DATA_SIZE: usize = 64;
const ALLOW_COUNT: u8 = 3;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    runtime::init_app();
    let args = Args::parse();
    let network = sdk::Network::from_string(&args.network);
    println!("network is : {:?}", network);
    let pair = sdk::init_solana_wallet().unwrap();
    let pub_key = pair.pubkey();

    println!(
        "server pub_key address : {:?} , check and wait balance...",
        pair.pubkey()
    );
    let connection = sdk::get_rpc_client(&network);
    sdk::confirm_balance(&connection, &network, &pub_key, 5);
    let program_account = runtime::program_account(args.program.clone());
    if !sdk::program_available(&connection, &program_account) {
        println!("program account is not available , exit...");
        return Ok(());
    }

    println!("now sol is ready , create one account for ddmonitor... ");

    let queue_name = args.name;
    let queue_pub = sdk::pda_queue_account(&program_account, &queue_name);
    println!("queue account is : {:?}", queue_pub);

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
        let queue_size = models::Queue::queue_size(DATA_SIZE, ALLOW_COUNT);
        let lamports = connection
            .get_minimum_balance_for_rent_exemption(queue_size)
            .unwrap();

        println!("queue pda need sol: {}", lamports);

        let accounts = vec![
            AccountMeta::new(pub_key.clone(), true),
            AccountMeta::new(queue_pub.clone(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        // Create the instruction by serializing our instruction data via borsh
        let instruction = Instruction::new_with_borsh(
            runtime::program_account(args.program.clone()),
            &InstructionData::RegisterQueue {
                name: queue_name.to_string(),
                data_size: DATA_SIZE,
                allow_count: ALLOW_COUNT,
            },
            accounts,
        );

        let blockhash = connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

        println!("create queue request send ...");
        match connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                println!("create queue account tx : {:?}", tx);
                if args.allow != "default" {
                    println!("no allow account , exit...");
                    return Ok(());
                }
            }
            Err(err) => {
                let _transaction_err = err.get_transaction_error().unwrap();
                println!("create queue account error : {:?}", _transaction_err);
                println!("create queue account error : {:?}", err);
            }
        }
    } else {
        println!("queue account is exist , skip create...");
    }

    sdk::get_account_updates(&network, &queue_pub, handlers::main).unwrap();

    Ok(())
}
