# Solana HTTP Server

A Rust-based HTTP server that provides REST API endpoints for Solana blockchain operations including keypair generation, token operations, message signing, and transaction creation.

## Setup Instructions

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/codeaj001/http_server
cd http_server
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
Leave it running and open another terminal, then continue with the following:

1. Install test dependencies:
```bash
npm install
```
2. Run the tests:
```bash
npm test
```
