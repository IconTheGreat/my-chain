#[derive(Debug)]
pub struct Transaction {
    pub nonce: u8,
    pub gas_price: u64,
    pub gas: u64,
    pub to: String,
    pub value: u64,
    pub v: String,
    pub r: String,
    pub s: String,
    // pub input: String,
}

impl Transaction {
    pub fn new (to: String, amount: u64) -> Self {
        Transaction {
            nonce: 0,
            gas_price: 0,
            gas: 0,
            to,
            value: amount,
            v: 0.to_string(),
            r: 0.to_string(),
            s: 0.to_string(), 
        }
    }
}