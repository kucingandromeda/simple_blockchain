pub mod transaction{
    use std::clone::Clone;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Transaction{
        pub from_address:Option<String>,
        pub to_address:Option<String>,
        pub amount:usize,
    }

    impl Transaction {
        pub fn new(from_address:Option<String>, to_address:Option<String>, amount:usize)-> Self{
            Transaction{
                from_address,
                to_address,
                amount
            }
        }

        pub fn get_hash(&self)-> String{
            let transaction_json = serde_json::to_string(self)
                .expect("\n error translating transaction to json in walte\n");

            let hash = openssl::sha::sha256(transaction_json.as_bytes());
            hex::encode(hash)
        }
    }
}