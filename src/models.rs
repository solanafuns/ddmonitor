use {
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    solana_program::pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MathStuffSum {
    pub sum: u32,
    pub owner: Pubkey,
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
