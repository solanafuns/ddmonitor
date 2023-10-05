use borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    RegisterQueue {
        name: String,
        data_size: usize,
        allow_count: u8,
    },
    PushMessage {
        name: String,
        data: String,
    },
    AddUserPub {
        name: String,
        user_pub: String,
    },
}
