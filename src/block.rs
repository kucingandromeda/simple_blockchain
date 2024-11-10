pub mod block{
    use std::{clone::Clone, fmt::Debug};
    use openssl::sha::sha256;
    use hex;
    use chrono;
    use crate::transaction::transaction::Transaction;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    #[warn(dead_code)]
    enum PEM{
        Private(String),
        Signature(Vec<u8>)
    }

    #[allow(unused)]
    impl PEM {
        fn unwrap_pem(&self){
            match self {
                PEM::Private(value) => println!("{}", value),
                PEM::Signature(_) => ()
            }
        }

        fn unwrap_signature(&self)-> Option<Vec<u8>>{
            match self{
                PEM::Signature(sign) => Some(sign.clone()),
                PEM::Private(_) => None
            }
        }
    }

    impl Debug for PEM {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PEM")
                .field("Private", &"---")
                .field("Signature", &"Signature")
                .finish()
        }
    }


    #[warn(dead_code)]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Block{
        pub transaction:Vec<Transaction>,
        pub previous_hash:Option<String>,
        pub hash:Option<String>,
        pub count:usize,
        time:String,
    }

    impl Block {
        
        pub fn new(transaction:Vec<Transaction>) -> Self{
            Block{
                transaction,
                previous_hash:None,
                hash:None,
                count:0,
                time:format!("{}", chrono::Local::now()),
            }
        }

        pub fn hash_generator(&self)-> String{
            let mut copy_block = self.clone();
            copy_block.hash = None;

            let sha = sha256(format!("{}{}",self.previous_hash
                    .clone()
                    .expect("previos hash not found"), self.count)
                .as_bytes());
            hex::encode(sha)
        }

        pub fn mining_time(&mut self,difficult:usize ,fill:i32){
            if fill > 9 {
                panic!("fill greater than 9");
            };

            let mut fill_string = Vec::new();
            fill_string.resize(difficult, fill);
            let fill_string:Vec<String> = fill_string.iter().map(|value| value.to_string()).collect();
            let fill_string = &fill_string.join("")[..];
            
            self.hash = Some(self.hash_generator());
            while &self.hash.clone().unwrap()[0..difficult] != fill_string {
                self.hash = Some(self.hash_generator());
                self.count += 1;
            }
        }


    }

}