pub mod block{
    use std::sync::Arc;


    #[derive(Debug)]
    pub struct Block{
        pub name: String,
        pub value: isize,
        pub previous_hash:Option<String>,
        pub hash:Option<String>
    }

    impl Block {
        
        pub fn new(name:String, value:isize) -> Self{
            Block{
                name,
                value,
                previous_hash:None,
                hash:None
            }
        }

    }

}