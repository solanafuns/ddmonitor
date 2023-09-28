use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

use crate::{instruction::InstructionData, process};

declare_id!("9pW59BsNCqtQC1xucwTXYS4Qe9qz5AgSy2jajE63odQb");

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Program ID: {:?}", program_id);
    match InstructionData::try_from_slice(instruction_data) {
        Ok(instruction_data) => {
            msg!("InstructionData: {:?}", instruction_data);
            match instruction_data {
                InstructionData::RegisterQueue { name, data_size } => {
                    msg!("RegisterQueue: {:?}", name);
                    return process::do_create_queue(accounts, &name, data_size, program_id);
                }
                _ => {
                    msg!("not implemented");
                }
            }
        }
        Err(err) => {
            msg!("Error: {:?}", err);
        }
    }
    Ok(())
}
