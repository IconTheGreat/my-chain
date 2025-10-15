mod block;
mod blockchain;
mod wallet;
mod transaction;

fn main() {
    let mut chain = blockchain::Blockchain::new();
    let tx1 = transaction::Transaction::new("address1".into(), 50);
    let tx2 = transaction::Transaction::new("address2".into(), 75);
    chain.add_block(vec![tx1, tx2]);
    println!("{:?}", chain);
    println!("{:?}", chain.get_latest_block());
    let wallet = wallet::Wallet::new();
    println!("Wallet Address: {:?}", wallet.get_address());
    println!("New Wallet: {:?}", wallet);
    let message = "Hello, blockchain!".as_bytes();
    let signature = wallet.sign(message);
    println!("Signature: {:?}", signature);
}
