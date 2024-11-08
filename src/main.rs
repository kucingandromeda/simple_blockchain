mod block;
mod chain;

use block::block::Block;

use crate::chain::chain::Chain;

fn main(){
    let mut chain = Chain::new();

    let yazem_block = Block::new("yazem".to_string(), 10);
    chain.add_block(yazem_block);

    // let rio_block = Block::new("rio".to_string(), 10000);
    // chain.add_block(rio_block);
    // println!("{:#?}", chain.chain)
}