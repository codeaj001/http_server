use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use bs58;
use serde::Serialize;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub fn generate_keypair() -> KeypairResponse {
    let keypair = Keypair::new();
    KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    }
}

