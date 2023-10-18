use ddmonitor::ddmonitor_init;

use {
    clap::Parser,
    ddmonitor::{handlers, runtime, sdk},
    env_logger::Env,
    log::{error, info},
    solana_sdk::signer::Signer,
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
    #[arg(short, long, default_value_t = String::from(runtime::DEFAULT_PROGRAM))]
    program: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    ddmonitor_init!();
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

    sdk::send_instruction(
        &connection,
        &pub_key,
        &vec![&pair],
        sdk::create_instruction(
            pub_key.clone(),
            queue_pub.clone(),
            args.program.clone(),
            args.name.clone(),
            args.message.clone().into_bytes(),
        ),
    );

    sdk::send_instruction(
        &connection,
        &pub_key,
        &vec![&pair],
        sdk::create_instruction(
            pub_key.clone(),
            queue_pub.clone(),
            args.program.clone(),
            args.name.clone(),
            handlers::ActionInfo::ActionSample(1, 2).wrapper(),
        ),
    );

    loop {
        info!("you will write these lines to this queue: -> {}", args.name);
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
                    queue_pub.clone(),
                    args.program.clone(),
                    args.name.clone(),
                    handlers::ActionInfo::Raw(line).wrapper(),
                ),
            );
        }
    }

    Ok(())
}
