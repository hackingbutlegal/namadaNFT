# NamadaNFT Standard

A set of programs featuring an on-chain NFT smart contract and an off-chain client for interacting with the Namada blockchain. This project aims to implement a custom NFT standard on Namada using Rust and WASM, with an integrated client to mint, transfer, and query NFTs. It is organized as a Cargo workspace containing two crates: one for the NFT contract and one for the client application.

## Table of Contents

- [Overview](#overview)
- [Folder Structure](#folder-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Project](#building-the-project)
- [Testing](#testing)
- [Deployment](#deployment)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

This project consists of two main parts:

- **nft_contract:** The on-chain smart contract written in Rust, which implements a robust NFT standard including metadata, privacy, royalties, and transfer logic. The contract compiles to WebAssembly (WASM) to be deployed on Namada.
- **nft_client:** An off-chain client application that leverages the Namada SDK to interact with the deployed NFT contract. It provides functionalities to mint NFTs, transfer them, and query tokens from a wallet.

The project is designed to work on Namada’s testnet and mainnet. It follows modern Rust best practices and provides a clear example of how to structure blockchain projects using a Cargo workspace.

## Folder Structure

```
namadaNFT/
├── Cargo.toml               # Workspace manifest listing all crates
├── README.md                # This file
├── nft_contract/            # On-chain NFT contract (smart contract)
│   ├── Cargo.toml           # Contract crate manifest and WASM target settings
│   └── src/
│       ├── lib.rs           # Main entry point and re-exports for NFT contract logic
│       ├── nft.rs           # NFT definitions, implementations, and logic
│       └── tests.rs         # Unit and integration tests for the contract
└── nft_client/              # Off-chain client application
    ├── Cargo.toml           # Client crate manifest with Namada SDK dependencies
    └── src/
        ├── main.rs          # Entry point for the client application
        ├── explorer.rs      # (Optional) Module for blockchain explorer integration
        └── tests.rs         # Integration tests for client functionality
```

## Prerequisites

Before getting started, ensure you have the following installed:

- **Rust:**  
  [Install Rust](https://www.rust-lang.org/tools/install) and update it:

  ```bash
  rustup update
  ```

- **WASM Target:**  
  Add the WASM target required for compiling smart contracts:

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **Namada CLI:**  
  Follow the instructions on [Namada’s GitHub](https://github.com/anoma/namada) to install the Namada CLI for interacting with the blockchain.

- **Wallet Setup:**  
  Create a Namada wallet (using the Namada CLI or another recommended tool) and secure your wallet file for signing transactions.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your_username/my_nft_project.git
   cd my_nft_project
   ```

2. **Workspace Setup:**  
   The repository uses a Cargo workspace. The root `Cargo.toml` declares the workspace members:
   
   ```toml
   [workspace]
   members = [
       "nft_contract",
       "nft_client",
   ]
   ```

## Building the Project

### Build the NFT Contract (WASM)

1. **Navigate to the Contract Directory:**

   ```bash
   cd nft_contract
   ```

2. **Compile for WASM:**

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. **(Optional) Optimize the WASM Binary:**  
   Use `wasm-opt` from Binaryen to optimize the binary:

   ```bash
   wasm-opt -Oz target/wasm32-unknown-unknown/release/nft_contract.wasm -o nft_contract_opt.wasm
   ```

### Build the Client Application

1. **Navigate to the Client Directory:**

   ```bash
   cd ../nft_client
   ```

2. **Build the Client:**

   ```bash
   cargo build --release
   ```

## Testing

### Running On-Chain Contract Tests

From the root or within the `nft_contract` folder, run:

```bash
cargo test --manifest-path nft_contract/Cargo.toml
```

### Running Client Tests

Similarly, from the client folder, execute:

```bash
cargo test --manifest-path nft_client/Cargo.toml
```

These tests include unit tests and integration tests using dummy implementations and the Tokio runtime to simulate asynchronous blockchain interactions.

## Deployment

### Deployment to Namada Testnet/Mainnet

1. **Configure Endpoints:**  
   Ensure your client code is configured to use the appropriate Namada endpoints. For example, for testnet use:
   
   - RPC Endpoint: `https://testnet-rpc.namada.network:9090`
   - Explorer URL: `https://testnet-explorer.namada.network`

   For mainnet, update these endpoints accordingly (e.g., `https://mainnet-rpc.namada.network:9090`).

2. **Deploy the Contract:**  
   Use the Namada CLI to deploy your optimized WASM binary. For example:

   ```bash
   namada deploy nft_contract_opt.wasm --from <your_wallet_address> --chain-id mainnet --gas 1000000
   ```

   Replace `<your_wallet_address>` with your actual address and adjust gas values as needed.

3. **Verify Deployment:**  
   Use Namada’s explorer or CLI commands to verify that your contract is deployed successfully.

## Usage

Once your contract is deployed, you can use the client application to interact with it.

### Minting an NFT

Run the client application with the correct arguments. For example:

```bash
cargo run --release --package nft_client
```

This will:
- Connect to Namada’s blockchain using your configured endpoints.
- Mint a new NFT with metadata and optional royalty configuration.
- Print the minted token ID and display NFT details from your wallet.

### Explorer Integration

The client includes explorer integration in `explorer.rs`. You can view your NFTs on the Namada explorer using the generated URLs.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes with clear commit messages.
4. Open a pull request describing your changes.

Before contributing, please review our coding style and test guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

For further questions or support, please refer to the [Namada documentation](https://github.com/anoma/namada) or open an issue in the repository.

Happy coding!
```
```markdown
````
