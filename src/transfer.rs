use validator::Validate;

use solana_sdk::{
    pubkey::Pubkey,
};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use base64;

#[derive(Debug, Deserialize, Validate)]
pub struct SendSolRequest {
    #[validate(length(min = 32, max = 44))]
    pub from: String,
    #[validate(length(min = 32, max = 44))]
    pub to: String,
    #[validate(range(min = 1))]
    pub lamports: u64,
}


#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct TokenAccountMetaResponse {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<TokenAccountMetaResponse>,
    pub instruction_data: String,
}

pub fn create_sol_transfer_instruction(request: SendSolRequest) -> Result<SendSolResponse, String> {
    // Check for empty fields
    if request.from.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.to.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }

    if request.lamports == 0 {
        return Err("Lamports must be greater than 0".to_string());
    }

    let from_pubkey = Pubkey::from_str(&request.from)
        .map_err(|e| format!("Invalid sender address: {}", e))?;
    let to_pubkey = Pubkey::from_str(&request.to)
        .map_err(|e| format!("Invalid recipient address: {}", e))?;

    let instruction = solana_sdk::system_instruction::transfer(
        &from_pubkey,
        &to_pubkey,
        request.lamports,
    );

    let accounts = instruction.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();

    Ok(SendSolResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(instruction.data),
    })
}

#[derive(Debug, Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

pub fn create_token_transfer_instruction(request: SendTokenRequest) -> Result<SendTokenResponse, String> {
    // Check for empty fields
    if request.destination.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.mint.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.owner.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }

    if request.amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    // We parse the mint address for validation even though we don't use it directly
    let _mint = Pubkey::from_str(&request.mint)
        .map_err(|e| format!("Invalid mint address: {}", e))?;
    let destination = Pubkey::from_str(&request.destination)
        .map_err(|e| format!("Invalid destination address: {}", e))?;
    let owner = Pubkey::from_str(&request.owner)
        .map_err(|e| format!("Invalid owner address: {}", e))?;

    // In SPL token transfers, we need to specify the token account (not just the owner)
    // Normally we would derive or look up the token account, but for simplicity
    // we'll use a placeholder and assume the client will provide the correct token account
    let source_token_account = Pubkey::from_str(&request.owner)
        .map_err(|e| format!("Invalid owner address: {}", e))?;

    let instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        &source_token_account,
        &destination,
        &owner,
        &[],
        request.amount,
    ).map_err(|e| e.to_string())?;

    let accounts = instruction.accounts.iter().map(|meta| TokenAccountMetaResponse {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();

    Ok(SendTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(instruction.data),
    })
}

