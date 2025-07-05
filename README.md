# Solana HTTP Server

A Rust-based HTTP server that provides REST API endpoints for Solana blockchain operations including keypair generation, token operations, message signing, and transaction creation.

## Features

- **Keypair Generation**: Generate new Solana keypairs
- **Message Signing & Verification**: Sign and verify messages using ed25519
- **Token Operations**: Create tokens and mint tokens
- **Transfer Operations**: Create SOL and SPL token transfer instructions

## Setup Instructions

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd solana_http_server
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:8080` by default.

## API Endpoints

### POST /keypair
Generate a new Solana keypair.

**Response:**
```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

### POST /message/sign
Sign a message with a private key.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base58-encoded-signature",
    "pubkey": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

### POST /message/verify
Verify a signed message.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "signature": "base58-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```

### POST /token/create
Create a new SPL token mint instruction.

**Request:**
```json
{
  "mintAuthority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "decimals": 6
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [...],
    "instruction_data": "base58-encoded-instruction-data"
  }
}
```

### POST /token/mint
Create a mint_to instruction for SPL tokens.

**Request:**
```json
{
  "mint": "base58-encoded-public-key",
  "destination": "base58-encoded-public-key",
  "authority": "base58-encoded-public-key",
  "amount": 1000000
}
```

### POST /send/sol
Create a SOL transfer instruction.

**Request:**
```json
{
  "from": "base58-encoded-public-key",
  "to": "base58-encoded-public-key",
  "lamports": 1000000
}
```

### POST /send/token
Create an SPL token transfer instruction.

**Request:**
```json
{
  "destination": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "owner": "base58-encoded-public-key",
  "amount": 1000000
}
```

## Testing

To run the provided test suite, you'll need Node.js and the required dependencies:

1. Install test dependencies:
```bash
npm install jest axios tweetnacl @solana/web3.js bs58 @solana/spl-token
```

2. Create a jest config file (`jest.config.js`):
```javascript
module.exports = {
  testEnvironment: 'node',
  testTimeout: 30000,
};
```

3. Start the server:
```bash
cargo run
```

4. Run the tests in another terminal:
```bash
npx jest test-file.js
```

## Error Handling

All endpoints return proper HTTP status codes:
- `200` - Success
- `400` - Bad Request (invalid input parameters)
- `404` - Not Found (invalid endpoint)

Error responses include a `success: false` field and an `error` message describing the issue.

## Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **ed25519-dalek**: Ed25519 cryptography
- **solana-sdk**: Solana blockchain SDK
- **spl-token**: SPL token program
- **bs58**: Base58 encoding

## Architecture

The server is structured into modules:
- `main.rs` - Server setup and routing
- `response.rs` - Response type definitions
- `routes/` - Individual endpoint implementations
  - `keypair.rs` - Keypair generation
  - `message.rs` - Message signing and verification
  - `token.rs` - Token operations
  - `transfer.rs` - Transfer operations

## Performance Notes

This implementation is optimized for:
- Fast cryptographic operations using ed25519-dalek
- Efficient JSON serialization with serde
- Proper error handling for all edge cases
- Memory-efficient base58 encoding
- Consistent response formats matching test expectations

## Security Considerations

- Private keys are handled securely in memory
- All input validation is performed before processing
- Proper error messages without exposing internal details
- Type-safe handling of cryptographic operations

