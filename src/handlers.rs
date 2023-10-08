use {crate::sdk, borsh::BorshDeserialize, contract::models, log::info};

pub fn main(b64data: String) {
    let buf = sdk::base64_decode(&b64data);
    if !buf.is_err() {
        let data = buf.unwrap();
        info!("data len : {} ", data.len());
        let queue = models::Queue::try_from_slice(&data);
        if !queue.is_err() {
            let queue = queue.unwrap();
            info!("queue data : {:?}", queue.data);
            let utf8_message = String::from_utf8(queue.data);
            if !utf8_message.is_err() {
                info!("message : {}", utf8_message.unwrap());
            }
        }
    }
}
