use borsh::to_vec;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::operator;

pub fn do_create_queue(
    accounts: &[AccountInfo],
    seed_str: &str,
    data_size: usize,
    program_id: &Pubkey,
) -> ProgramResult {
    msg!("you will create one Queue account with name : {}", seed_str);
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let queue_account = next_account_info(account_info_iter)?;
    let system_account = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[&program_id.clone().to_bytes(), seed_str.as_bytes()],
        program_id,
    );

    assert!(
        payer.is_signer && payer.is_writable && queue_account.is_writable,
        "users invalid!"
    );

    if pda != *queue_account.key {
        msg!("Queue account does not have the correct pda");
        return Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    if queue_account.owner != program_id {
        msg!("Queue account does not have the correct program id");
        return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
    }

    if queue_account.data_len() != 0 {
        msg!("Queue account already in use");
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    } else {
        msg!("init Queue data");
        let q = operator::Queue::new_queue(payer.key, &vec![], data_size);
        let q_data = to_vec(&q).unwrap();
        let pda_space: u64 = q_data.len() as u64;

        // 计算所需的租金
        let rent = Rent::get()?;
        let rent_lamports: u64 = rent.minimum_balance(pda_space as usize);

        // 创建账户
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                queue_account.key,
                rent_lamports,
                pda_space,
                program_id,
            ),
            &[payer.clone(), queue_account.clone(), system_account.clone()],
            &[&[payer.key.as_ref(), seed_str.as_bytes(), &[bump_seed]]],
        )?;
        queue_account.data.borrow_mut().copy_from_slice(&q_data);
    }
    Ok(())
}
