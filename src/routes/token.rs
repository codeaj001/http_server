use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use bs58;

use solana_sdk::{pubkey::Pubkey, instruction::Instruction};
use spl_token::instruction::initialize_mint;
use spl_associated_token_account::get_associated_token_address;

use crate::response::{AppJson, success, bad_request};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaJson {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

pub async fn create_token(Json(body): Json<CreateTokenRequest>) -> AppJson {
    let mint_pubkey = match body.mint.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid mint pubkey"),
    };

    let mint_authority = match body.mint_authority.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid mint authority pubkey"),
    };

    let instruction: Instruction = match initialize_mint(
        &spl_token::ID,
        &mint_pubkey,
        &mint_authority,
        None,
        body.decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => return bad_request(&format!("Failed to build instruction: {}", e)),
    };

    let accounts: Vec<AccountMetaJson> = instruction.accounts.iter().map(|meta| {
        AccountMetaJson {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    success(json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": bs58::encode(&instruction.data).into_string(),
    }))
}


#[derive(Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

pub async fn mint_token(Json(body): Json<MintTokenRequest>) -> AppJson {
    let mint = match body.mint.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid mint pubkey"),
    };

    let dest = match body.destination.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid destination pubkey"),
    };

    let authority = match body.authority.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => return bad_request("Invalid authority pubkey"),
    };

    // Get associated token address for the destination
    let dest_ata = get_associated_token_address(&dest, &mint);

    let instruction = match spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint,
        &dest_ata,
        &authority,
        &[],
        body.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return bad_request(&format!("Instruction creation failed: {}", e)),
    };

    let accounts: Vec<AccountMetaJson> = instruction.accounts.iter().map(|meta| {
        AccountMetaJson {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    success(json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": bs58::encode(&instruction.data).into_string()
    }))
}

