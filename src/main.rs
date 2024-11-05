use openssl::sha::sha256;
use hex;
use std::clone::Clone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Block{
    index:usize,
    time:String,
    data:usize,
    previos_hash:Option<String>,
    hash: Option<String>
}


impl Block {
    fn new(index:usize, time:String, data:usize) -> Block{
        Block{
            index,
            time,
            data,
            previos_hash: None,
            hash: None
        }
    }

    fn calculate_sha(&self) -> String{
        hex::encode(sha256(format!("{}{}{}{}", self.index, self.time, self.data, self.previos_hash.clone().expect("error here")).as_bytes()))
    }
}

//

struct BlockChain{
    chain:Vec<Block>
}

impl BlockChain {
    fn init() -> BlockChain{

        let mut genesis = Block::new(0, "2024".to_string(), 10);
        genesis.previos_hash = Some("00000".to_string());
        genesis.hash = Some(genesis.calculate_sha());

        BlockChain { 
            chain: vec![genesis]
         }
    }

    fn show_chain(&self)-> Vec<Block>{
        self.chain.clone()
    }

    fn get_latest_block(&self) -> Block{
        let list = &self.chain[self.chain.len() - 1];
        list.clone()
    }

    fn add_block(&mut self,mut block:Block){
        block.previos_hash = self.get_latest_block().hash;
        block.hash = Some(block.calculate_sha());
        self.chain.push(block);
    }

    fn validation_chain(&self) -> bool{
        for i in 1..self.chain.len(){

            let current_block = self.chain[i].clone();
            let previos_block = self.chain[i - 1].clone();

            if current_block.clone().hash.unwrap() != current_block.calculate_sha(){
                return false;
            }

            if current_block.previos_hash.unwrap() != previos_block.hash.unwrap(){
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut neo_coin = BlockChain::init();
    
    let a = Block::new(1, "2024".to_string(), 100);
    neo_coin.add_block(a);

    let b = Block::new(2, "2025".to_string(), 80);
    neo_coin.add_block(b);

    let c = Block::new(3, "2026".to_string(), 8090);
    neo_coin.add_block(c);

    let status = neo_coin.validation_chain();
    println!(" validation => {}", status);

    let chain = neo_coin.show_chain();
    println!("{chain:#?}");
}
