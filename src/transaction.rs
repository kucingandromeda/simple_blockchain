pub mod transaction{
    use std::clone::Clone;
    use serde::{Deserialize, Serialize};
    use  serde_json::{Deserializer, Serializer};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Transaction{
        pub from_address:Option<String>,
        pub to_address:Option<String>,
        pub amount:usize,
    }

    impl Transaction {
        pub fn new(from_address:Option<String>, to_address:Option<String>, amount:usize)-> Self{
            Transaction{
                from_address:from_address,
                to_address:to_address,
                amount
            }
        }
    }
}