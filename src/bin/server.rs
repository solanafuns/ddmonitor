use {
    ddmonitor::{runtime, sdk},
    solana_sdk::signer::Signer,
    std::{thread, time},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    runtime::init_app();
    let pair = sdk::init_solana_wallet().unwrap();
    let pub_key = pair.pubkey();
    println!("server address : {:?}", pair.pubkey());
    println!("check and wait balance");

    loop {
        let connection = sdk::get_rpc_client();
        let balance = connection.get_balance(&pub_key).unwrap();
        println!("current balance is : {}", balance);
        if balance >= runtime::LAMPORTS_PER_SOL {
            break;
        } else {
            let delay = time::Duration::from_secs(3);
            thread::sleep(delay);
        }
    }

    println!("now sol is ready , create one account for ddmonitor");

    Ok(())
}
