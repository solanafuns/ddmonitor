use {
    borsh::{self, to_vec, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    solana_program::pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    RegisterQueue {
        name: String,
        data_size: usize,
        allow_count: u8,
    },
    PushMessage {
        name: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerPrivate {
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionBlockResponseOptions {
    show_input: bool,
    show_raw_input: bool,
    show_effects: bool,
    show_events: bool,
    show_object_changes: bool,
    show_balance_changes: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Queue {
    pub creator: Pubkey,
    pub allow: Vec<Pubkey>,
    pub data: Vec<u8>,
}

impl Queue {
    pub fn new_queue(creator: &Pubkey, allow: &Vec<Pubkey>, data_size: usize) -> Self {
        let data: Vec<u8> = vec![0; data_size];
        Self {
            creator: creator.clone(),
            allow: allow.clone(),
            data,
        }
    }

    pub fn queue_size(data_size: usize, allow_count: u8) -> usize {
        let mut allow_list = Vec::new();
        let mut mut_index = allow_count;
        while mut_index > 0 {
            allow_list.push(Pubkey::new_unique());
            mut_index -= 1;
        }
        let tmp_queue = Self::new_queue(&Pubkey::new_unique(), &allow_list, data_size);
        let q_data = to_vec(&tmp_queue).unwrap();
        q_data.len()
    }
}
