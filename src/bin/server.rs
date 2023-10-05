use {
    ddmonitor::{models, runtime, sdk},
    solana_program::{instruction::AccountMeta, system_program},
    solana_sdk::{instruction::Instruction, signer::Signer, transaction::Transaction},
    std::{thread, time},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    runtime::init_app();
    let pair = sdk::init_solana_wallet().unwrap();
    let pub_key = pair.pubkey();
    println!("server pub_key address : {:?}", pair.pubkey());
    println!("check and wait balance...");
    let connection = sdk::get_rpc_client();
    loop {
        let balance = connection.get_balance(&pub_key).unwrap();
        println!("current balance is : {}", balance);
        if balance >= runtime::LAMPORTS_PER_SOL {
            break;
        } else {
            let delay = time::Duration::from_secs(3);
            thread::sleep(delay);
        }
    }

    println!("now sol is ready , create one account for ddmonitor... ");
    const DATA_SIZE: usize = 1024;
    const ALLOW_COUNT: u8 = 10;
    const QUEUE_NAME: &str = "hello";

    let queue_account = sdk::pda_queue_account(QUEUE_NAME);
    println!("queue account is : {:?}", queue_account);
    let queue_size = models::Queue::queue_size(DATA_SIZE, ALLOW_COUNT);
    let lamports = connection
        .get_minimum_balance_for_rent_exemption(queue_size)
        .unwrap();

    println!("need sol: {}", lamports);

    // The accounts required by both our on-chain program and the system program's
    // `create_account` instruction, including the vault's address.
    let accounts = vec![
        AccountMeta::new(pub_key.clone(), true),
        AccountMeta::new(queue_account.clone(), false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    // Create the instruction by serializing our instruction data via borsh
    let instruction = Instruction::new_with_borsh(
        runtime::program_account(),
        &models::InstructionData::RegisterQueue {
            name: QUEUE_NAME.to_string(),
            data_size: DATA_SIZE,
            allow_count: ALLOW_COUNT,
        },
        accounts,
    );

    let blockhash = connection.get_latest_blockhash().unwrap();

    let transaction =
        Transaction::new_signed_with_payer(&[instruction], Some(&pub_key), &[&pair], blockhash);

    match connection.send_and_confirm_transaction(&transaction) {
        Ok(tx) => {
            println!("create queue account tx : {:?}", tx);
        }
        Err(err) => {
            let _transaction_err = err.get_transaction_error().unwrap();
            println!("create queue account error : {:?}", _transaction_err);
            println!("create queue account error : {:?}", err);
        }
    }
    Ok(())
}
