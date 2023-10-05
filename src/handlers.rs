use {
    crate::{models, sdk},
    borsh::BorshDeserialize,
};

pub fn main(b64data: String) {
    let buf = sdk::base64_decode(&b64data);
    if !buf.is_err() {
        let data = buf.unwrap();
        println!("data len : {} ", data.len());
        let queue = models::Queue::try_from_slice(&data);
        if !queue.is_err() {
            let queue = queue.unwrap();
            println!("queue data : {:?}", queue.data);
        }
    }
}
