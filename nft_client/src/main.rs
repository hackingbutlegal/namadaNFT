use std::collections::HashMap;
use std::str::FromStr;
use namada_sdk::{
    Account, Client, NamadaClient, Transaction, Address, Token,
};
use namada_core::{
    hash::Hash,
    token::Amount,
};
// Assume these types are defined in your NFT module.
use nft_module::{
    NftMetadata, 
    RoyaltyConfig, 
    NftAction, 
    NftToken, 
    TokenType, 
    PrivacyConfig, 
    VisibilityLevel,
};

/// NFT Minting Client for Namada.
pub struct NftMintClient {
    client: NamadaClient,
    wallet: Account,
}

impl NftMintClient {
    /// Creates a new NFT minting client.
    pub async fn new(
        grpc_endpoint: &str, 
        wallet_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = NamadaClient::connect(grpc_endpoint).await?;
        let wallet = Account::load_from_file(wallet_path)?;
        Ok(Self { client, wallet })
    }
    
    /// Mints a new NFT by building, signing, and submitting the minting transaction.
    pub async fn mint_nft(
        &self, 
        collection_address: Address,
        metadata: NftMetadata,
        royalty_config: Option<RoyaltyConfig>,
    ) -> Result<Hash, Box<dyn std::error::Error>> {
        // Build the NFT minting transaction.
        let tx = Transaction::new()
            .with_action(NftAction::Mint {
                collection: collection_address,
                metadata: metadata.clone(),
                royalty_config,
            })
            .sign(&self.wallet);
        
        // Submit the transaction and await the receipt.
        let tx_hash = self.client.submit_transaction(tx).await?;
        let receipt = self.client.wait_for_tx(tx_hash).await?;
        
        if receipt.status.is_success() {
            Ok(tx_hash)
        } else {
            Err("NFT minting transaction failed".into())
        }
    }
    
    /// Retrieves the list of NFT tokens associated with the client's wallet.
    pub async fn get_wallet_nfts(&self) -> Result<Vec<NftToken>, Box<dyn std::error::Error>> {
        let nft_tokens: Vec<NftToken> = self.client
            .query_account_tokens(self.wallet.address())
            .into_iter()
            .filter(|token| token.token_type == TokenType::Nft)
            .collect();
        Ok(nft_tokens)
    }
    
    /// Transfers an NFT to a recipient.
    pub async fn transfer_nft(
        &self,
        token_id: Hash,
        recipient: Address,
        sale_price: Option<Amount>,
    ) -> Result<Hash, Box<dyn std::error::Error>> {
        let tx = Transaction::new()
            .with_action(NftAction::Transfer {
                token_id,
                recipient,
                sale_price,
            })
            .sign(&self.wallet);
        
        let tx_hash = self.client.submit_transaction(tx).await?;
        let receipt = self.client.wait_for_tx(tx_hash).await?;
        
        if receipt.status.is_success() {
            Ok(tx_hash)
        } else {
            Err("NFT transfer transaction failed".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the NFT client using Namada's testnet endpoints.
    let client = NftMintClient::new(
        "https://testnet-rpc.namada.network:9090", // Testnet gRPC endpoint.
        "/path/to/wallet.json",
    ).await?;
    
    // Define the collection address (update with a valid testnet address).
    let collection_address = Address::from_str("namada1collection").unwrap();
    
    // Prepare NFT metadata.
    let metadata = NftMetadata {
        token_id: Hash::default(), // Will be generated on-chain.
        name: "My First Namada NFT".to_string(),
        description: Some("An example NFT on Namada Testnet".to_string()),
        uri: Some("ipfs://example-cid".to_string()),
        creator: client.wallet.address(),
        attributes: HashMap::from([
            ("rarity".to_string(), "common".to_string())
        ]),
        transferable: true,
        privacy_config: Some(PrivacyConfig {
            encrypted: false,
            encryption_key: None,
            visibility: VisibilityLevel::Public,
        }),
    };
    
    // Optional royalty configuration.
    let royalty_config = RoyaltyConfig {
        creator: client.wallet.address(),
        royalty_percentage: 500, // 5%
        secondary_recipients: vec![],
        royalty_token: None,
    };
    
    // Mint the NFT.
    let token_id = client.mint_nft(
        collection_address,
        metadata,
        Some(royalty_config),
    ).await?;
    
    println!("NFT minted successfully. Token ID: {:?}", token_id);
    
    // Retrieve and display wallet NFTs.
    let wallet_nfts = client.get_wallet_nfts().await?;
    for nft in wallet_nfts {
        println!("NFT: {}", nft.name);
        println!("Token ID: {:?}", nft.token_id);
    }
    
    // Explorer Integration Example.
    let explorer = NamadaExplorer::new("https://testnet-explorer.namada.network");
    println!("View NFT on Explorer: {}", explorer.get_nft_token_url(&token_id));
    
    Ok(())
}

/// Blockchain Explorer integration to generate URLs for tokens and wallet NFT collections.
pub struct NamadaExplorer {
    base_url: String,
}

impl NamadaExplorer {
    /// Creates a new explorer instance.
    pub fn new(base_url: &str) -> Self {
        Self { base_url: base_url.to_string() }
    }
    
    /// Returns a URL pointing to the NFT token page.
    pub fn get_nft_token_url(&self, token_id: &Hash) -> String {
        format!("{}/token/{}", self.base_url, token_id)
    }
    
    /// Returns a URL for viewing a wallet's NFT collection.
    pub fn get_wallet_nfts_url(&self, address: &Address) -> String {
        format!("{}/address/{}/nfts", self.base_url, address)
    }
}

/// Example Explorer usage.
fn display_nft_on_explorer() {
    let explorer = NamadaExplorer::new("https://testnet-explorer.namada.network");
    let token_id = Hash::from([0u8; 32]); // Example token ID.
    let wallet_address = Address::from_str("namada1wallet").unwrap();
    
    println!("View NFT: {}", explorer.get_nft_token_url(&token_id));
    println!("View Wallet NFTs: {}", explorer.get_wallet_nfts_url(&wallet_address));
}
