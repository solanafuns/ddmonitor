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
        data: Vec<u8>,
    },
    UserPubOperation {
        name: String,
        user_pub: String,
        allow: bool,
    },
}
