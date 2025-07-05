use serde_json::json;
use solana_sdk::signature::{Keypair, Signer};
use crate::response::{AppJson, success};

pub async fn generate_keypair() -> AppJson {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    let secret = keypair.to_bytes();
    
    success(json!({
        "pubkey": pubkey.to_string(),
        "secret": secret.to_vec()
    }))
}
