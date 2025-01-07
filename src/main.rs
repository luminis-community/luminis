mod blockchain;

use blockchain::{Block, Blockchain};

fn main() {
    let mut luminis = Blockchain::new();

    // Add block vào blockchain
    luminis.add_block("Khối dữ liệu thứ nhất".to_string());
    luminis.add_block("Khối dữ liệu thứ hai".to_string());

    // Print all blocks in the blockchain
    for block in luminis.chain.iter() {
        println!("{:?}", block);
    }
}
