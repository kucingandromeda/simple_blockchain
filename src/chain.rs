pub mod chain{

    use crate::block::block::Block;
    use openssl::sha::sha256;
    use hex;

    pub struct Chain{
        pub chain: Vec<Block>
    }

    impl Chain{

        pub fn new() -> Self{

            let mut genesis_block = Block::new("genesis".to_string(), 0);
            genesis_block.previous_hash = Some("00000".to_string());
            genesis_block.hash = Some(genesis_block.hash_generator());

            Chain{
                chain:vec![genesis_block]
            }
        }

        fn get_latest_block(&self) -> Block{
            let target = &self.chain[self.chain.len() - 1];
            target.clone()
        }

        pub fn add_block(&mut self, mut block:Block){

         let previos_block = self.get_latest_block();
         let previos_hash = previos_block.hash;
        
         block.previous_hash = previos_hash;
         block.hash = Some(block.hash_generator());
        
         block.mining_time(2, 0);
         self.chain.push(block);
        }

    }

}