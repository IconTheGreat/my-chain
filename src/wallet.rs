use ed25519_dalek_fiat::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Wallet { keypair }
    }

    pub fn get_address(&self) -> String {
        let public_key_bytes = self.keypair.public.to_bytes();
        hex::encode(public_key_bytes)
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let signature: Signature = self.keypair.sign(message);
        signature
    }
}