use {
    crate::sdk,
    borsh::{BorshDeserialize, BorshSerialize},
    contract::models,
    log::{error, info},
    solana_program::pubkey::Pubkey,
};

pub fn chat_main(b64data: String) {
    let buf = sdk::base64_decode(&b64data);
    if !buf.is_err() {
        let data = buf.unwrap();
        let queue = models::Queue::try_from_slice(&data);
        if !queue.is_err() {
            let queue = queue.unwrap();
            let utf8_message = String::from_utf8(queue.data.clone());
            if !utf8_message.is_err() {
                info!("message : {}", utf8_message.unwrap());
            }
            ActionInfo::from(queue.data).do_action();
        }
    }
}

pub fn main(b64data: String) {
    let buf = sdk::base64_decode(&b64data);
    if !buf.is_err() {
        let data = buf.unwrap();
        info!("data len : {} ", data.len());
        let queue = models::Queue::try_from_slice(&data);
        if !queue.is_err() {
            let queue = queue.unwrap();
            info!("queue last change : {:?} ", queue.last_change);
            info!("queue data : {:?} ", queue.data);
            let utf8_message = String::from_utf8(queue.data.clone());
            if !utf8_message.is_err() {
                info!("message : {}", utf8_message.unwrap());
            }
            ActionInfo::from(queue.data).do_action();
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ActionInfo {
    Raw(String),
    ActionSample(u8, u8),
    UserMessage(Pubkey, String),
    None,
}

impl ActionInfo {
    pub fn do_action(&self) {
        info!("you will do action : {:?}", &self);
        match &self {
            ActionInfo::ActionSample(x, y) => {
                info!("this is example action ! x =  {} , y = {}", x, y);
            }
            ActionInfo::Raw(msg) => {
                info!("this is raw action ! msg =  {} ", msg);
            }
            ActionInfo::UserMessage(user, msg) => {
                info!("🔥 user:{}  🔥 send msg:  {}", user.to_string(), msg);
            }
            ActionInfo::None => {
                error!("invalid action");
            }
        }
    }
}

impl Into<Vec<u8>> for ActionInfo {
    fn into(self) -> Vec<u8> {
        info!("wrapper action : {:?}", self);
        let mut v = self.try_to_vec().unwrap();
        let mut x = borsh::BorshSerialize::try_to_vec(&(v.len() as u32)).unwrap();
        x.append(&mut v);
        x
    }
}

impl From<Vec<u8>> for ActionInfo {
    fn from(payload: Vec<u8>) -> Self {
        let mut buf = payload.clone();
        let len = u32::try_from_slice(&buf[0..4]).unwrap() as usize;
        buf.drain(0..4);
        if buf.len() < len {
            error!("expect buffer too long : {}", len);
            ActionInfo::None
        } else {
            let data = buf[0..len].to_vec();
            info!("unwrap data : {:?}", data);
            let action = ActionInfo::try_from_slice(&data).unwrap();
            info!("unwrap action : {:?}", action);
            action
        }
    }
}
