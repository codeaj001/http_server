use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use bs58;

use ed25519_dalek::{Keypair, Signer, Signature, PublicKey};

use crate::response::{AppJson, success, bad_request};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    signature: String,
    public_key: String,
    message: String,
}

pub async fn sign_message(Json(body): Json<SignMessageRequest>) -> AppJson {
    if body.message.is_empty() || body.secret.is_empty() {
        return bad_request("Missing required fields");
    }

    let secret_bytes = match bs58::decode(&body.secret).into_vec() {
        Ok(bytes) if bytes.len() == 64 => bytes,
        _ => return bad_request("Invalid secret key format"),
    };

    let secret_array: [u8; 64] = match secret_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return bad_request("Invalid secret key length"),
    };

    let keypair = match Keypair::from_bytes(&secret_array) {
        Ok(kp) => kp,
        Err(_) => return bad_request("Invalid secret key format"),
    };
    
    let signature = keypair.sign(body.message.as_bytes());

    success(json!({
        "signature": bs58::encode(signature.to_bytes()).into_string(),
        "pubkey": bs58::encode(keypair.public.to_bytes()).into_string(),
        "message": body.message
    }))
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn verify_message(Json(body): Json<VerifyMessageRequest>) -> AppJson {
    if body.message.is_empty() || body.signature.is_empty() || body.pubkey.is_empty() {
        return bad_request("Missing required fields");
    }

    let signature_bytes = match bs58::decode(&body.signature).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return bad_request("Invalid base58 signature"),
    };

    let pubkey_bytes = match bs58::decode(&body.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return bad_request("Invalid base58 pubkey"),
    };

    if pubkey_bytes.len() != 32 || signature_bytes.len() != 64 {
        return bad_request("Invalid lengths for pubkey or signature");
    }

    let pubkey_array: [u8; 32] = match pubkey_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return bad_request("Invalid pubkey length"),
    };

    let verifying_key = match PublicKey::from_bytes(&pubkey_array) {
        Ok(key) => key,
        Err(_) => return bad_request("Invalid verifying key"),
    };

    let signature_array: [u8; 64] = match signature_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return bad_request("Invalid signature length"),
    };

    let signature = match Signature::from_bytes(&signature_array) {
        Ok(sig) => sig,
        Err(_) => return bad_request("Invalid signature format"),
    };

    let is_valid = verifying_key.verify_strict(body.message.as_bytes(), &signature).is_ok();

    success(json!({
        "valid": is_valid,
        "message": body.message,
        "pubkey": body.pubkey
    }))
}
