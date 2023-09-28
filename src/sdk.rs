use borsh::{self, BorshDeserialize, BorshSerialize};

pub fn hello(msg: &str) {
    println!("hello world {} !", msg);
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    RegisterQueue { name: String },
    PushMessage { name: String },
    Empty(u8),
}
