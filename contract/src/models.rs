use {
    borsh::{to_vec, BorshDeserialize, BorshSerialize},
    solana_program::{clock, pubkey::Pubkey, sysvar::Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Queue {
    pub creator: Pubkey,
    pub allow: Vec<Pubkey>,
    pub data: Vec<u8>,
    pub need_data_size: usize,
    pub created_at: i64,
    pub last_change: i64,
}

impl Queue {
    pub fn new_queue(creator: &Pubkey, allow: &Vec<Pubkey>, data_size: usize) -> Self {
        let clock = clock::Clock::get().unwrap();
        let data: Vec<u8> = vec![0; data_size];
        Self {
            creator: creator.clone(),
            allow: allow.clone(),
            need_data_size: data_size,
            data,
            created_at: clock.unix_timestamp,
            last_change: clock.unix_timestamp,
        }
    }
    pub fn push_data(&mut self, sender_pub: Pubkey, data: Vec<u8>) -> bool {
        if self.allow.contains(&sender_pub) {
            self.data = data;
            self.pad_to_length(self.need_data_size, 0);
            self.last_change = clock::Clock::get().unwrap().unix_timestamp;
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
        self.last_change = clock::Clock::get().unwrap().unix_timestamp;
    }

    fn pad_to_length(&mut self, desired_length: usize, padding_value: u8) {
        let current_length = self.data.len();
        if current_length < desired_length {
            let padding_amount = desired_length - current_length;
            self.data.extend(vec![padding_value; padding_amount]);
        }
    }

    pub fn queue_size(data_size: usize, allow_count: u8) -> usize {
        let allow: Vec<Pubkey> = vec![Pubkey::default(); allow_count as usize];
        let data = vec![0; data_size];
        let tmp_queue = Self {
            creator: Pubkey::new_unique(),
            need_data_size: data_size,
            allow,
            data,
            created_at: 0,
            last_change: 0,
        };
        to_vec(&tmp_queue).unwrap().len()
    }
}
