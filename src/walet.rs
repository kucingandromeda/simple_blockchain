pub mod walet{
    use std::fmt::Debug;
    use crate::transaction::transaction::Transaction;
    use openssl::rsa::{self, Padding};

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Walet{
        name:String,
        private_key:String,
        pub public_key:String
    }

    impl Debug for Walet{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Walet")
                .field("private_key", &"***")
                .field("public_key", &"public key")
                .finish()
        }
    }

    impl Walet{
        pub fn new(name:String)-> Self{
            
            let walet_rsa = rsa::Rsa::generate(2042)
                .expect("\nerror create rsa on walet\n");

            let public_key = walet_rsa.public_key_to_pem()
                .expect("\n error get public key on walet \n");
            let private_key = walet_rsa.private_key_to_pem()
                .expect("\n error get private key in walet \n");

            let public_key_hex = hex::encode(public_key);
            let private_key_hex = hex::encode(private_key);

            Walet{
                name,
                public_key:public_key_hex,
                private_key:private_key_hex,
            }
        }

        pub fn send_payment(&self, amount:usize, to_public_key:&String)-> (Transaction, Vec<u8>){
            let transaction = Transaction::new(Some(self.public_key.clone()), Some(to_public_key.clone()), amount);
            let hash = transaction.get_hash();

            let private_key = hex::decode( &self.private_key)
                .expect("\n error decode private key in walet");

            let walet_rsa = rsa::Rsa::private_key_from_pem(&private_key)
                .expect("\n error read private key in walet");

            let mut signature = vec![0; walet_rsa.size() as usize];
            walet_rsa.private_encrypt(&hash.as_bytes(), &mut signature, Padding::PKCS1)
                .expect("\n error create signature in walet \n");
            
            (transaction, signature)

        }
    }
}