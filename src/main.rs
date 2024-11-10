mod block;
mod chain;
mod transaction;
mod walet;

use crate::chain::chain::Chain;
use crate::transaction::transaction::Transaction;
use crate::walet::walet::Walet;

fn main(){
    let mut chain = Chain::new();

    // walet
    let yazem_walet = Walet::new("yazem".to_string());
    let rio = Walet::new("rio".to_string());
    
    // transaction
    let (mut yazem_transaction, yazem_signature)= yazem_walet.send_payment(100, &rio.public_key);
    // yazem_transaction.amount = 200;s
    let (rio_transaction, rio_signature) = rio.send_payment(50, &yazem_walet.public_key);

    chain.create_transaction(yazem_transaction, yazem_signature)
        .unwrap_or_else(|msg| println!("{}", msg));

    chain.create_transaction(rio_transaction, rio_signature)
        .unwrap_or_else(|msg| println!("{}", msg));





    // let yazem_transaction = Transaction::new("", to_address, amount)

    // let yazem_block = Block::new("yazem".to_string(), 10);
    // chain.add_block(yazem_block);

    // let salman_block = Block::new("salman".to_string(), 10);
    // chain.add_block(salman_block);

    println!("{:#?}", chain.pending_transaction)
}