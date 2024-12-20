pub mod chain{

    use openssl::rsa::Padding;

    use crate::transaction::transaction::Transaction;
    use crate::block::block::Block;

    pub struct Chain{
        pub chain: Vec<Block>,
        pub pending_transaction: Vec<Transaction>,
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
                mining_reward:100,
                pending_transaction:Vec::new(),
                difficult:2,
                fill:0,
            }
        }

        fn get_latest_block(&self) -> Block{
            let target = &self.chain[self.chain.len() - 1];
            target.clone()
        }

        pub fn create_transaction(&mut self, transaction:Transaction, signature:Vec<u8>)-> Result<(), String>{
            let hash = transaction.get_hash().clone();
            let public_key = transaction.from_address
                .clone()
                .expect("\n public key not found when verify signature \n");
            let public_key = hex::decode(public_key)
                .expect("\n error on decode public_key in verify signature");
            let sign_rsa = openssl::rsa::Rsa::public_key_from_pem(&public_key)
                .expect("\n error set public key while verify signature \n");

            let mut signature_decrypted = vec![0;sign_rsa.size() as usize];
            sign_rsa.public_decrypt(&signature, &mut signature_decrypted, Padding::PKCS1)
                .expect("\n error decrypted signature while verify signature \n");

            if hash != String::from_utf8_lossy(&signature_decrypted)[..64]{
                // println!("bad");
                return Err("\n hash and signature not same! \n".to_string());
            }

            // println!("ok");

            self.pending_transaction.push(transaction);
            Ok(())


        }

        pub fn mine_pending_transaction(&mut self, mining_reward_address:String){
            let mut block = Block::new(self.pending_transaction.clone());

            let previous_hash = self.get_latest_block().hash
                .clone()
                .expect("previos hash not found");

            block.previous_hash = Some(previous_hash);
            block.mining_time(self.difficult, self.fill as i32);

            println!("mining sucsess");
            block.transaction.push(
                Transaction::new(None, Some(mining_reward_address), self.mining_reward)
            );
            self.chain.push(block);
        }

        pub fn get_balance_of_address(&self,address:String)-> isize{
            let mut balance:isize = 0;

            for block in &self.chain{
                for trans in &block.transaction{
                    

                    if let Some(trans_address) = &trans.from_address {
                        if trans_address == &address{
                            balance -= trans.amount as isize;
                        }    
                    }

                    if let Some(trans_address) = &trans.to_address{
                        if trans_address == &address{
                            balance += trans.amount as isize;
                        }
                    }
                }

            }
            return balance;
        }
    }

}