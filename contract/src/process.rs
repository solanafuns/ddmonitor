use {
    crate::models,
    borsh::{to_vec, BorshDeserialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        pubkey::Pubkey,
        system_instruction,
        sysvar::{rent::Rent, Sysvar},
    },
};

pub fn do_create_queue(
    accounts: &[AccountInfo],
    seed_str: &str,
    data_size: usize,
    allow_count: u8,
    program_id: &Pubkey,
) -> ProgramResult {
    msg!("You will create one Queue account with name : {}", seed_str);
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let queue_account = next_account_info(account_info_iter)?;
    let system_account = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[seed_str.as_bytes()], program_id);

    msg!(
        "from client account info: system_account: {:?} , pda: {:?},bump_seed : {} ",
        system_account,
        pda,
        bump_seed
    );

    assert!(
        payer.is_signer && payer.is_writable && queue_account.is_writable,
        "users invalid!"
    );

    if pda != *queue_account.key {
        msg!("Queue account does not have the correct pda");
        return Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    if queue_account.data_len() != 0 {
        msg!("Queue account already in use");
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    } else {
        msg!("curent queue_account has data,try to unserialize...");

        let mut allow_keys = Vec::new();

        for _ in 0..allow_count {
            allow_keys.push(system_account.key.clone());
        }

        allow_keys[0] = payer.key.clone();

        let data_queue = models::Queue::new_queue(&payer.key, &allow_keys, data_size);
        let q_data = to_vec(&data_queue).unwrap();
        let pda_space: u64 = q_data.len() as u64;

        // 计算所需的租金
        let rent = Rent::get()?;
        let rent_lamports: u64 = rent.minimum_balance(pda_space as usize);
        msg!("rent_lamports: {}", rent_lamports);
        //  创建账户
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                queue_account.key,
                rent_lamports,
                pda_space,
                program_id,
            ),
            &[payer.clone(), queue_account.clone(), system_account.clone()],
            &[&[seed_str.as_bytes(), &[bump_seed]]],
        )?;
        queue_account.data.borrow_mut().copy_from_slice(&q_data);
    }
    Ok(())
}

pub fn do_push_message(
    accounts: &[AccountInfo],
    seed_str: &str,
    message_data: &str,
    program_id: &Pubkey,
) -> ProgramResult {
    msg!(
        "You will push one message to Queue account with name : {}",
        seed_str
    );
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let queue_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;
    let system_account: &AccountInfo<'_> = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[seed_str.as_bytes()], program_id);

    msg!(
        "from client account info: system_account: {:?} , pda: {:?},bump_seed : {} ",
        system_account,
        pda,
        bump_seed
    );

    assert!(
        payer.is_signer
            && payer.is_writable
            && queue_account.is_writable
            && queue_account.owner == program_id,
        "users invalid!"
    );

    if pda != *queue_account.key {
        msg!("Queue account does not have the correct pda");
        return Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    let mut user_queue = models::Queue::try_from_slice(&queue_account.data.borrow())?;
    user_queue.push_data(payer.key.clone(), String::from(message_data).into_bytes());
    msg!("current data is : {:?}", user_queue.data);
    let q_data = to_vec(&user_queue).unwrap();

    queue_account.data.borrow_mut().copy_from_slice(&q_data);

    Ok(())
}

pub fn do_add_user_pub(
    _accounts: &[AccountInfo],
    _seed_str: &str,
    _user_pub: &str,
    _program_id: &Pubkey,
) -> ProgramResult {
    Ok(())
}
