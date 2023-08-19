mod structures;
mod sst;

use crate::structures::MemBlock;

fn main() {
    println!("Hello, world!");
    let mut block = MemBlock::new();
    block.insert(vec!(0, 1), vec!(1, 2));
    println!("{}", block.size());
}
