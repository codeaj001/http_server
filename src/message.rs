use solana_sdk::{
    signature::{Signature, Signer},
    signer::keypair::Keypair,
    pubkey::Pubkey,
};
use std::str::FromStr;
use bs58;
use base64;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

pub fn sign_message(request: SignMessageRequest) -> Result<SignMessageResponse, String> {
    if request.message.is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.secret.is_empty() {
        return Err("Missing required fields".to_string());
    }

    let secret_bytes = bs58::decode(&request.secret)
        .into_vec()
        .map_err(|e| format!("Invalid secret key: {}", e))?;
    
    // Check if the byte length is correct for a keypair
    if secret_bytes.len() != 64 {
        return Err(format!("Invalid keypair length: expected 64 bytes, got {}", secret_bytes.len()));
    }

    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|e| format!("Failed to create keypair: {}", e))?;
    
    let signature = keypair.sign_message(request.message.as_bytes());
    
    Ok(SignMessageResponse {
        signature: base64::encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: request.message,
    })
}

#[derive(Debug, Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub fn verify_message(request: VerifyMessageRequest) -> Result<VerifyMessageResponse, String> {
    if request.message.is_empty() {
        return Err("Missing required fields".to_string());
    }

    if request.signature.is_empty() {
        return Err("Missing required fields".to_string());
    }

    if request.pubkey.is_empty() {
        return Err("Missing required fields".to_string());
    }

    let pubkey = Pubkey::from_str(&request.pubkey)
        .map_err(|e| format!("Invalid public key: {}", e))?;
    
    let signature_bytes = base64::decode(&request.signature)
        .map_err(|e| format!("Invalid signature: {}", e))?;
    
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {}", e))?;
    
    let valid = signature.verify(pubkey.as_ref(), request.message.as_bytes());
    
    Ok(VerifyMessageResponse {
        valid,
        message: request.message,
        pubkey: request.pubkey,
    })
}

