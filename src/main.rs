mod block;
mod chain;
mod transaction;


use block::block::Block;
use openssl::{bn::BigNum, rsa::{self, Padding}};

use crate::chain::chain::Chain;
use crate::transaction::transaction::Transaction;

fn main(){
    let mut chain = Chain::new();

    // let yazem_transaction = Transaction::new("", to_address, amount)

    // let yazem_block = Block::new("yazem".to_string(), 10);
    // chain.add_block(yazem_block);

    // let salman_block = Block::new("salman".to_string(), 10);
    // chain.add_block(salman_block);

    // println!("{:#?}", chain.chain)
}