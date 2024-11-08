pub mod block{
    use std::clone::Clone;
    use openssl::{pkey::Private, rsa::{self, Padding, Rsa}, sha::sha256};
    use hex;


    #[derive(Debug, Clone)]
    pub struct Block{
        pub name: String,
        pub value: isize,
        pub previous_hash:Option<String>,
        pub hash:Option<String>,
        pub count:usize,
        // RSA
        pub digital_signature:Option<String>,
        pub public_pem:Option<Vec<u8>>,
        rsa_block:Option<Rsa<Private>>
    }

    impl Block {
        
        pub fn new(name:String, value:isize) -> Self{

            Block{
                name,
                value,
                previous_hash:None,
                hash:None,
                count:0,
                digital_signature:None,
                public_pem:None,
                rsa_block:None,
            }
        }

        pub fn hash_generator(&self)-> String{
            let sha = sha256(format!("{}{}{}{}", self.name, self.value, self.previous_hash.clone().unwrap(), self.count).as_bytes());
            hex::encode(sha)
        }

        fn digital_signature_generator(&mut self){
            let rsa_block = rsa::Rsa::generate(2042).expect("error create RSA");
            
            // pem
            let public_pem = rsa::RsaRef::public_key_to_pem(&rsa_block)
                .expect("error create public_pem");

            // let private_pem = rsa::RsaRef::private_key_to_pem(&rsa_block)
            //     .expect("error create private_pem");
            // pem

            let mut encrypted = vec![0; rsa_block.size() as usize];

            let hash = self.hash.clone().expect("hash not found for create digital signature");
            rsa_block.private_encrypt(hash.as_bytes(), &mut encrypted, Padding::PKCS1)
                .expect("error created digital signature");

            // set up RSA in block
            self.public_pem = Some(public_pem);
            self.rsa_block = Some(rsa_block);
            self.digital_signature = Some(hex::encode(encrypted));   
        }

        pub fn validation_digital_signature_fn(&self){
            // hash
            let hash = self.hash_generator();
            println!("{}", hash);

            // hash_decrypted
            let digital_signatur = hex::decode(self.digital_signature
                    .clone()
                    .expect("digital signature not found"))
                        .expect("error decode from hex");

            let mut decrypted = vec![0; self.rsa_block.clone().expect("rsa_fn not found").size() as usize];

            let public_pem = self.public_pem.clone().unwrap();
            let rsa_decrypted = rsa::Rsa::public_key_from_pem(&public_pem).unwrap();
            rsa_decrypted.public_decrypt(&digital_signatur, &mut decrypted, Padding::PKCS1)
                .expect("error for decrypted digital signature for validation");

            println!("\n{}", hex::encode(decrypted));
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
            self.validation_digital_signature_fn();
            println!("\n mining clear:{} | digital signature has created \n", self.hash.clone().unwrap());
        }


    }

}