mod model;

use crate::model::memblock::MemBlock;
use crate::model::sst::SST;

fn main() {
    println!("Hello, world!");
    let mut block = MemBlock::new();
    block.insert(vec!(0, 1), vec!(1, 2));
    println!("{}", block.size());
}
