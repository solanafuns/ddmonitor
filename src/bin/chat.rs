use ddmonitor::ddmonitor_init;

use {
    clap::Parser,
    contract::instruction::InstructionData,
    ddmonitor::{handlers, runtime, sdk},
    env_logger::Env,
    log::{error, info},
    solana_program::{instruction::AccountMeta, system_program},
    solana_sdk::{instruction::Instruction, transaction::Transaction},
    std::thread,
};

/// One chat app use ddmonitor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the queue
    #[arg(short, long, default_value_t = String::from("creator706"))]
    room: String,

    /// Network to communicate with
    #[arg(long, default_value_t = String::from("local"))]
    network: String,

    /// Solana program address
    #[arg(short, long, default_value_t = String::from(runtime::DEFAULT_PROGRAM))]
    program: String,

    /// Room member address to add to the room
    #[arg(short, long, default_value_t = String::from(""))]
    add_user: String,

    /// Start chat with room members
    #[arg(short, long, default_value_t = false)]
    chat_start: bool,
}

const DATA_SIZE: usize = 1024;
const ALLOW_COUNT: u8 = 5;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    ddmonitor_init!();
    let args: Args = Args::parse();
    info!("ddmonitor init checking...");

    let (network, pair, pub_key, connection, program_account) =
        sdk::ddmonitor_init(&args.network, &args.program)?;

    let room_account = sdk::pda_queue_account(&program_account, &args.room);
    info!("room account is : {}", &room_account);
    let queue_avaliable = {
        let room_info = connection.get_account(&room_account);
        if room_info.is_err() {
            false
        } else {
            let room_info = room_info.unwrap();
            room_info.owner == args.program.parse().unwrap()
                && room_info.lamports > 0
                && !room_info.executable
                && room_info.data.len() > 0
        }
    };
    if !queue_avaliable {
        info!("room account is not avaliable, now create room account !");

        let accounts = vec![
            AccountMeta::new(pub_key.clone(), true),
            AccountMeta::new(room_account.clone(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        // Create the instruction by serializing our instruction data via borsh
        let instruction = Instruction::new_with_borsh(
            runtime::program_account(args.program.clone()),
            &InstructionData::RegisterQueue {
                name: args.room.to_string(),
                data_size: DATA_SIZE,
                allow_count: ALLOW_COUNT,
            },
            accounts,
        );

        let blockhash = connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

        info!("create room request send ...");
        match connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                info!("create room account tx : {:?}", tx);
            }
            Err(err) => {
                let _transaction_err = err.get_transaction_error().unwrap();
                error!("create room account error : {:?}", _transaction_err);
                error!("create room account error : {:?}", err);
            }
        }
    }

    if args.add_user != "" {
        info!("add user :{} to the room {}", &args.add_user, &args.room);

        let accounts = vec![
            AccountMeta::new(pub_key.clone(), true),
            AccountMeta::new(room_account.clone(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        // Create the instruction by serializing our instruction data via borsh
        let instruction = Instruction::new_with_borsh(
            runtime::program_account(args.program.clone()),
            &InstructionData::UserPubOperation {
                name: args.room.to_string(),
                user_pub: args.add_user.parse().unwrap(),
                allow: true,
            },
            accounts,
        );

        let blockhash = connection.get_latest_blockhash().unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

        info!("add user to  room request send ...");
        match connection.send_and_confirm_transaction(&transaction) {
            Ok(tx) => {
                info!("add user to  room  tx : {:?}", tx);
            }
            Err(err) => {
                let _transaction_err = err.get_transaction_error().unwrap();
                error!("add user to  room  error : {:?}", _transaction_err);
                error!("add user to  room error : {:?}", err);
            }
        }
    }

    if args.chat_start {
        info!("chat start ...");

        sdk::send_instruction(
            &connection,
            &pub_key,
            &vec![&pair],
            sdk::create_instruction(
                pub_key.clone(),
                room_account.clone(),
                args.program.clone(),
                args.room.clone(),
                handlers::ActionInfo::UserMessage(pub_key.clone(), "I'm in!".to_string()).wrapper(),
            ),
        );

        thread::spawn(move || sdk::get_account_updates(&network, &room_account, handlers::main));

        loop {
            info!("you will write these lines to this queue: -> {}", args.room);
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            if line.trim() == "exit" {
                break;
            } else {
                sdk::send_instruction(
                    &connection,
                    &pub_key,
                    &vec![&pair],
                    sdk::create_instruction(
                        pub_key.clone(),
                        room_account.clone(),
                        args.program.clone(),
                        args.room.to_string(),
                        handlers::ActionInfo::UserMessage(pub_key.clone(), line).wrapper(),
                    ),
                );
            }
        }
    }

    Ok(())
}
