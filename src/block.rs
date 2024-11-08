pub mod block{
    use std::sync::Arc;
    use std::clone::Clone;
    use openssl::sha::sha256;
    use hex;


    #[derive(Debug, Clone)]
    pub struct Block{
        pub name: String,
        pub value: isize,
        pub previous_hash:Option<String>,
        pub hash:Option<String>,
        pub count:usize,
    }

    impl Block {
        
        pub fn new(name:String, value:isize) -> Self{
            Block{
                name,
                value,
                previous_hash:None,
                hash:None,
                count:0
            }
        }

        pub fn hash_generator(&self)-> String{
            let sha = sha256(format!("{}{}{}{}", self.name, self.value, self.previous_hash.clone().unwrap(), self.count).as_bytes());
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

            println!("mining clear:{}", self.hash.clone().unwrap());
        }

    }

}