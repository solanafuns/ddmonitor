use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

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
    pub fn push_data(&mut self, sender_pub: Pubkey, data: Vec<u8>) -> bool {
        if self.allow.contains(&sender_pub) {
            self.data = data;
            return true;
        }
        false
    }

    pub fn add_push_pub(&mut self, user: &Pubkey) {
        for i in 0..self.allow.len() {
            if self.allow[i] == Pubkey::default() {
                self.allow[i] = user.clone();
                break;
            }
        }
    }
}
