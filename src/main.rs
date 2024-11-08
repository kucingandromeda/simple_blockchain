mod block;
mod chain;


use block::block::Block;
use openssl::{bn::BigNum, rsa::{self, Padding}};

use crate::chain::chain::Chain;

fn main(){
    // let mut chain = Chain::new();

    let rsa_fn = openssl::rsa::Rsa::generate(2042).expect("failed generate key");

    // pem
    let public_pem = openssl::rsa::RsaRef::public_key_to_pem(&rsa_fn).expect("error on get public_pem");
    // println!("private : {:#?}", String::from_utf8_lossy(&public_pem)); // public
    let private_pem = openssl::rsa::RsaRef::private_key_to_pem(&rsa_fn).expect("error on get public_pem");
    // println!("private : {:#?}", String::from_utf8_lossy(&private_pem)); // private

    // test
    let text = "halo wkwkwkwk";
    let mut enc = vec![0; rsa_fn.size() as usize];
    let mut dec = vec![0; rsa_fn.size() as usize];

    let chiper = rsa_fn.public_encrypt(text.as_bytes(), &mut enc, Padding::PKCS1).unwrap();
    let dec_again = rsa_fn.private_decrypt(&enc, &mut dec, Padding::PKCS1).unwrap();
    println!("{}", String::from_utf8_lossy(&dec));


    // let chiper = rsa_fn.public_encrypt("halo".as_bytes(), to, Padding::NONE)

    // let a = BigNum::from_u32(2042).unwrap();
    // let b = BigNum::from_u32(2042).unwrap();
    // let c = BigNum::from_u32(2042).unwrap();
    // let a = rsa::RsaPrivateKeyBuilder::new(a,b,c).unwrap().build();

    // let yazem_block = Block::new("yazem".to_string(), 10);
    // chain.add_block(yazem_block);

    // let salman_block = Block::new("salman".to_string(), 10);
    // chain.add_block(salman_block);

    // println!("{:#?}", chain.chain)
}