use std::convert::Into;

#[derive(Debug, Clone)]
enum MyEnum {
    StringData(String),
    ByteData(u8),
}

impl Into<Vec<u8>> for MyEnum {
    fn into(self) -> Vec<u8> {
        match self {
            MyEnum::StringData(s) => s.as_bytes().to_vec(),
            MyEnum::ByteData(n) => vec![n],
        }
    }
}

fn transfer_data<T: Into<Vec<u8>>>(data: T) {
    println!("msg_body = {:?}", data.into());
}

fn main() {
    let mut enum_data = MyEnum::StringData("Hello".to_string());
    let msg_body: Vec<u8> = enum_data.into();
    println!("msg_body = {:?}", msg_body);
    enum_data = MyEnum::ByteData(0x01);
    transfer_data(enum_data.clone());
}
