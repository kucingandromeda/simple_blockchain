pub mod block{
    use std::clone::Clone;
    use openssl::{rsa::{self, Padding}, sha::sha256};
    use hex;
    use chrono;
    use crate::transaction::transaction::Transaction;
    use serde_json;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    #[warn(dead_code)]
    enum PEM{
        Private(String)
    }

    #[warn(dead_code)]
    impl PEM {
        fn unwrap_pem(&self){
            match self {
                Self::Private(value) => println!("{}", value)
            }
        }
    }

    impl std::fmt::Debug for PEM {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PEM")
                .field("Private",&"---")
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
        // RSA
        pub digital_signature:Option<Vec<u8>>,
        pub public_pem:Option<String>,
        private_pem:Option<PEM>,
        rsa_size:Option<u32>,

    }

    impl Block {
        
        pub fn new(transaction:Vec<Transaction>) -> Self{

            Block{
                transaction,
                previous_hash:None,
                hash:None,
                count:0,
                time:format!("{}", chrono::Local::now()),
                //RSA
                digital_signature:None,
                rsa_size:None,
                public_pem:None,
                private_pem:None,
            }
        }

        pub fn hash_generator(&self)-> String{
            let mut copy_block = self.clone();
            copy_block.hash = None;
            copy_block.digital_signature = None;
            copy_block.rsa_size = None;
            copy_block.public_pem = None;
            copy_block.private_pem = None;

            let json = serde_json::to_string(&copy_block)
                .expect("error create json from copy block");
            // println!("{}", json);

            let sha = sha256(format!("{}{}",self.previous_hash.clone().unwrap(), self.count).as_bytes());
            hex::encode(sha)
        }

        fn digital_signature_generator(&mut self){
            let rsa_block = rsa::Rsa::generate(2042).expect("error create RSA");
            
            // pem
            let public_pem = rsa::RsaRef::public_key_to_pem(&rsa_block)
                .expect("error create public_pem");

            let private_pem = rsa::RsaRef::private_key_to_pem(&rsa_block)
                .expect("error create private_pem");
            // pem

            let mut encrypted = vec![0; rsa_block.size() as usize];

            let hash = self.hash_generator();
            rsa_block.private_encrypt(hash.as_bytes(), &mut encrypted, Padding::PKCS1)
                .expect("error created digital signature");

            // set up RSA in block
            self.public_pem = Some(hex::encode(public_pem));
            self.private_pem = Some(PEM::Private(hex::encode(private_pem)));
            self.digital_signature = Some(encrypted);   
            self.rsa_size = Some(rsa_block.size());
        }

        pub fn verify_digital_signature_fn(&self)-> bool{
            // hash
            let hash = self.hash_generator();

            // hash_decrypted
            let digital_signatur = self.digital_signature
                .clone()
                .expect("digital signature not found");

            let mut decrypted = vec![0; self.rsa_size.expect("rsa_fn not found") as usize];

            let public_pem = hex::decode(self.public_pem.clone().unwrap())
                .unwrap();

            let rsa_decrypted = rsa::Rsa::public_key_from_pem(&public_pem).unwrap();
            rsa_decrypted.public_decrypt(&digital_signatur, &mut decrypted, Padding::PKCS1)
                .expect("error for decrypted digital signature for validation");

            
            let decrypted_cut_64bytes = &String::from_utf8_lossy(&decrypted).to_string()[..64];

            let hash = hash;

            if hash == decrypted_cut_64bytes{
                true
            } else {
                false
            }


            
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

            self.digital_signature_generator();
            self.verify_digital_signature_fn();
            // println!("{:?}", self.verify_digital_signature_fn());
            println!("\n mining clear:{} | digital signature has created \n", self.hash.clone().unwrap());
        }


    }

}