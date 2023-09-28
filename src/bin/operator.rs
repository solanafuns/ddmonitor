use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use ddmonitor::sdk::hello;
use ddmonitor::sdk::InstructionData;

fn main() {
    hello("Hello, ddmontor operator !");

    // let d = InstructionData::RegisterQueue {
    //     name: "hello world".to_string(),
    // };

    let d = InstructionData::Empty(0);

    let x = to_vec(&d).unwrap();
    print!("x: {:?}", x);

    const BUFFER_SIZE: usize = 128;
    let mut buf = [0u8; BUFFER_SIZE];
    println!("buffer : {:?}", buf);
    d.serialize(&mut &mut buf[..]).unwrap();
    println!("buffer : {:?}", buf);

    let x = InstructionData::try_from_slice(&buf).unwrap();
    println!("x: {:?}", x);
}
