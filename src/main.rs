mod block;
mod chain;


use block::block::Block;
use openssl::{bn::BigNum, rsa};

use crate::chain::chain::Chain;

fn main(){
    let mut chain = Chain::new();

    let a = BigNum::from_u32(2042).unwrap();
    let b = BigNum::from_u32(2042).unwrap();
    let c = BigNum::from_u32(2042).unwrap();
    let a = rsa::RsaPrivateKeyBuilder::new(a,b,c).unwrap().build();

    let yazem_block = Block::new("yazem".to_string(), 10);
    chain.add_block(yazem_block);

    let salman_block = Block::new("salman".to_string(), 10);
    chain.add_block(salman_block);

    println!("{:#?}", chain.chain)
}