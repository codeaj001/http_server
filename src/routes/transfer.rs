use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use bs58;

use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::response::{AppJson, success, bad_request};

#[derive(Deserialize)]
pub struct SolTransferRequest {
    from: String,
    to: String,
    lamports: u64,
}

pub async fn send_sol(Json(body): Json<SolTransferRequest>) -> AppJson {
    let from_pubkey = match body.from.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid sender public key"),
    };

    let to_pubkey = match body.to.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid recipient public key"),
    };

    if body.lamports == 0 {
        return bad_request("Amount must be greater than 0");
    }

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, body.lamports);

    success(json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": instruction.accounts.iter().map(|meta| meta.pubkey.to_string()).collect::<Vec<String>>(),
        "instruction_data": bs58::encode(&instruction.data).into_string()
    }))
}

#[derive(Deserialize)]
pub struct TokenTransferRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct TokenAccountMeta {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

pub async fn send_token(Json(body): Json<TokenTransferRequest>) -> AppJson {
    let destination = match body.destination.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid destination address"),
    };

    let mint = match body.mint.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid mint address"),
    };

    let owner = match body.owner.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid owner address"),
    };

    // Compute associated token accounts
    let source_ata = get_associated_token_address(&owner, &mint);
    let dest_ata = get_associated_token_address(&destination, &mint);

    // Create transfer instruction
    let instruction = match spl_token::instruction::transfer(
        &spl_token::ID,
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        body.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return bad_request(&format!("Instruction failed: {}", e)),
    };

    let accounts: Vec<TokenAccountMeta> = instruction.accounts.iter().map(|meta| TokenAccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();

    success(json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": bs58::encode(&instruction.data).into_string()
    }))
}
