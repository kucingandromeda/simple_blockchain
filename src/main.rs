mod block;
mod chain;

use block::block::Block;

use crate::chain::chain::Chain;

fn main(){
    let mut chain = Chain::new();

    let yazem_block = Block::new("yazem".to_string(), 10);
    chain.add_block(yazem_block);

    let salman_block = Block::new("salman".to_string(), 10);
    chain.add_block(salman_block);

    println!("{:#?}", chain.chain)
}