pub mod chain{

    use crate::block::block::Block;
    use openssl::sha::sha256;
    use hex;

    pub struct Chain{
        chain: Vec<Block>
    }

    impl Chain{

        pub fn new() -> Self{

            let mut genesis_block = Block::new("genesis".to_string(), 0);
            genesis_block.previous_hash = Some("00000".to_string());
            genesis_block.hash = Some(hash_generator(&genesis_block));
            println!("{:#?}", genesis_block);

            Chain{
                chain:Vec::new()
            }
        }

    }

    pub fn hash_generator(block:&Block) -> String{
        let sha = sha256(format!("{}{}{}", block.name, block.value, block.previous_hash.clone().unwrap()).as_bytes());
        hex::encode(sha)
    }

}