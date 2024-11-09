mod block;
mod chain;
mod transaction;

use crate::chain::chain::Chain;
use crate::transaction::transaction::Transaction;

fn main(){
    let mut chain = Chain::new();
    

    // yazem pay salman
    chain.create_transaction(
        Transaction::new(Some("yazem".to_string()), Some("salman".to_string()), 100)
    );

    // salman pay yazem
    chain.create_transaction(
        Transaction::new(Some("salman".to_string()), Some("yazem".to_string()), 50)
    );

    chain.mine_pending_transaction("yazem".to_string());

    // let balance = chain.get_balance_of_address("yazem".to_string()); 

    chain.create_transaction(
        Transaction::new(Some("yazem".to_string()), Some("salman".to_string()), 100)
    );

    // salman pay yazem
    chain.create_transaction(
        Transaction::new(Some("salman".to_string()), Some("yazem".to_string()), 50)
    );

    chain.mine_pending_transaction("yazem".to_string());

    // println!("yazem have => {}", balance);
    println!("{:#?}", chain.chain);
    // println!("{:#?}", chain.pending_transaction);


    // let yazem_transaction = Transaction::new("", to_address, amount)

    // let yazem_block = Block::new("yazem".to_string(), 10);
    // chain.add_block(yazem_block);

    // let salman_block = Block::new("salman".to_string(), 10);
    // chain.add_block(salman_block);

    // println!("{:#?}", chain.chain)
}