use {
    crate::{instruction::InstructionData, process},
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, msg,
        pubkey::Pubkey,
    },
};

fn dump_accounts_info(accounts: &[AccountInfo]) {
    msg!("Dump Account Info list... ");
    for (i, account) in accounts.iter().enumerate() {
        msg!("{}: {:?}", i, account);
    }
}

declare_id!("9pW59BsNCqtQC1xucwTXYS4Qe9qz5AgSy2jajE63odQb");

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Program ID: {:?}", program_id);
    dump_accounts_info(accounts);

    match InstructionData::try_from_slice(instruction_data) {
        Ok(instruction_data) => {
            msg!("InstructionData: {:?}", instruction_data);
            match instruction_data {
                InstructionData::RegisterQueue {
                    name,
                    data_size,
                    allow_count,
                } => {
                    msg!("RegisterQueue: {:?}", name);
                    return process::do_create_queue(
                        accounts,
                        &name,
                        data_size,
                        allow_count,
                        program_id,
                    );
                }
                InstructionData::PushMessage { name, data } => {
                    return process::do_push_message(accounts, &name, &data, program_id);
                }
                InstructionData::UserPubOperation {
                    name,
                    user_pub,
                    allow,
                } => {
                    return process::do_operate_user_pub(
                        accounts, &name, &user_pub, allow, program_id,
                    );
                }
            }
        }
        Err(err) => {
            msg!("Error: {:?}", err);
        }
    }
    Ok(())
}
