mod block;
mod blockchain;

fn main() {
    let mut chain = blockchain::Blockchain::new();
    chain.add_block("New Block".into());
    chain.add_block("Another Block".into());
    println!("{:?}", chain);
    println!("{:?}", chain.get_latest_block());
}
