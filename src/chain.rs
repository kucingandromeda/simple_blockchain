pub mod chain{

    use crate::transaction::transaction::Transaction;
    use crate::block::block::Block;
    use openssl::sha::sha256;
    use hex;

    pub struct Chain{
        pub chain: Vec<Block>,
        pending_transaction: Vec<Transaction>,
        mining_reward: usize,
        difficult:usize,
        fill:usize,
    }

    impl Chain{

        pub fn new() -> Self{

            let pending_transaction:Vec<Transaction> = Vec::new();

            let mut genesis_block = Block::new(pending_transaction);
            genesis_block.previous_hash = Some("00000".to_string());

            genesis_block.mining_time(2, 00);

            Chain{
                chain:vec![genesis_block],
                mining_reward:50,
                pending_transaction:Vec::new(),
                difficult:2,
                fill:0,
            }
        }

        fn get_latest_block(&self) -> Block{
            let target = &self.chain[self.chain.len() - 1];
            target.clone()
        }

        pub fn create_transaction(&mut self, transaction:Transaction){
            self.pending_transaction.push(transaction);
        }

        pub fn mine_pending_transaction(&mut self, mining_reward_address:String){
            let mut block = Block::new(self.pending_transaction.clone());
            block.mining_time(self.difficult, self.fill as i32);

            println!("mining sucsess");
            self.chain.push(block);
            self.pending_transaction.push(
                Transaction::new(None, Some(mining_reward_address), 50)
            );
        }

        pub fn get_balance_of_address(&self,address:String)-> usize{
            let mut balance = 0;

            for block in &self.chain{
                for trans in &block.transaction{

                    if let Some(trans_address) = &trans.from_address {
                        if trans_address == &address{
                            balance -= trans.amount;
                        }    
                    }

                    if let Some(trans_address) = &trans.to_address{
                        if trans_address == &address{
                            balance += trans.amount;
                        }
                    }
                }

            }
            return balance;
        }

        pub fn add_block(&mut self, mut block:Block){

         let previos_block = self.get_latest_block();
         let previos_hash = previos_block.hash;
        
         block.previous_hash = previos_hash;
         block.hash = Some(block.hash_generator());
        
         block.mining_time(2, 0);
        //  self.chain.push(block);
        }

    }

}