use solana_sdk::{
    pubkey::Pubkey,
};
use spl_token::instruction::initialize_mint;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use base64;

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

pub fn create_token_instruction(request: CreateTokenRequest) -> Result<CreateTokenResponse, String> {
    // Check for empty fields
    if request.mint_authority.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.mint.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    let mint_authority = Pubkey::from_str(&request.mint_authority)
        .map_err(|e| format!("Invalid mint authority: {}", e))?;
    let mint = Pubkey::from_str(&request.mint)
        .map_err(|e| format!("Invalid mint address: {}", e))?;

    let instruction = initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        request.decimals,
    ).map_err(|e| e.to_string())?;

    let accounts = instruction.accounts.iter().map(|meta| AccountMetaResponse {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    Ok(CreateTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(instruction.data),
    })
}

#[derive(Debug, Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

pub fn mint_token_instruction(request: MintTokenRequest) -> Result<CreateTokenResponse, String> {
    // Check for empty fields
    if request.mint.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.destination.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    if request.authority.trim().is_empty() {
        return Err("Missing required fields".to_string());
    }
    
    let mint = Pubkey::from_str(&request.mint)
        .map_err(|e| format!("Invalid mint address: {}", e))?;
    let destination = Pubkey::from_str(&request.destination)
        .map_err(|e| format!("Invalid destination address: {}", e))?;
    let authority = Pubkey::from_str(&request.authority)
        .map_err(|e| format!("Invalid authority address: {}", e))?;

    let instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        request.amount,
    ).map_err(|e| e.to_string())?;

    let accounts = instruction.accounts.iter().map(|meta| AccountMetaResponse {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    Ok(CreateTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(instruction.data),
    })
}

