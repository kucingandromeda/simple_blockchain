mod block;
mod chain;
mod transaction;
mod walet;

use crate::chain::chain::Chain;
use crate::walet::walet::Walet;

fn main(){
    let mut chain = Chain::new();

    // walet
    let yazem_walet = Walet::new("yazem".to_string());
    let rio = Walet::new("rio".to_string());
    
    // transaction
    let (yazem_transaction, yazem_signature)= yazem_walet.send_payment(10, &rio.public_key);

    // mine
    chain.create_transaction(yazem_transaction, yazem_signature)
        .unwrap_or_else(|msg| println!("{}", msg));

    // show balance
    chain.mine_pending_transaction(yazem_walet.public_key.clone());
    let balance = chain.get_balance_of_address(yazem_walet.public_key);
    println!("yazem coin is => {}", balance);

    // show chain
    // dbg!(chain.chain);

    // show pending transaction
    // dbg!(chain.pending_transaction);
}