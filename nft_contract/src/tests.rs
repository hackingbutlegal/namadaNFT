// ==========================
// On-chain Program Unit Tests
// ==========================
#[cfg(test)]
mod program_tests {
    use super::*;
    use std::str::FromStr;
    use std::collections::HashMap;

    #[test]
    fn test_nft_lifecycle() {
        // Setup addresses.
        let creator = Address::from_str("namada1creator").unwrap();
        let recipient = Address::from_str("namada1recipient").unwrap();
        let fee_collector = Address::from_str("namada1feecollector").unwrap();
        
        // Create NFT collection.
        let mut collection = NftCollection::new(
            "Test Collection".to_string(), 
            fee_collector.clone(), 
            10 // 0.1% fee
        );
        
        // Prepare metadata.
        let metadata = NftMetadata {
            token_id: Hash::default(), // Will be replaced by mint.
            name: "Test NFT".to_string(),
            description: Some("A comprehensive test NFT".to_string()),
            uri: Some("ipfs://example-cid".to_string()),
            creator: creator.clone(),
            attributes: HashMap::from([
                ("rarity".to_string(), "rare".to_string())
            ]),
            transferable: true,
            privacy_config: Some(PrivacyConfig {
                encrypted: false,
                encryption_key: None,
                visibility: VisibilityLevel::Public,
            }),
        };
        
        // Royalty configuration.
        let royalty_config = RoyaltyConfig {
            creator: creator.clone(),
            royalty_percentage: 500, // 5%
            secondary_recipients: vec![
                (creator.clone(), 250) // Additional 2.5%
            ],
            royalty_token: None,
        };
        
        // Mint NFT.
        let token_id = collection.mint(
            &mut TxContext::default(), 
            metadata, 
            Some(royalty_config)
        ).expect("Minting should succeed");
        
        // Transfer NFT.
        let outcome = collection.transfer(
            &mut TxContext::default(), 
            token_id,
            &creator, 
            &recipient, 
            Some(Amount::from(1_000_000u64))
        ).expect("Transfer should succeed");
        
        // Validate transfer outcome.
        assert!(outcome.royalties.is_some());
        assert!(outcome.program_fee.is_some());
        
        // Verify ownership update.
        let new_owner = collection.token_owners.get(&token_id).unwrap();
        assert_eq!(*new_owner, recipient);
    }
    
    #[test]
    fn test_unauthorized_transfer() {
        let creator = Address::from_str("namada1creator").unwrap();
        let unauthorized = Address::from_str("namada1unauthorized").unwrap();
        let recipient = Address::from_str("namada1recipient").unwrap();
        let fee_collector = Address::from_str("namada1feecollector").unwrap();
        
        let mut collection = NftCollection::new(
            "Test Collection".to_string(), 
            fee_collector,
            10
        );
        
        let metadata = NftMetadata {
            token_id: Hash::default(),
            name: "Test NFT".to_string(),
            description: Some("NFT for unauthorized transfer test".to_string()),
            uri: Some("ipfs://example-cid".to_string()),
            creator: creator.clone(),
            attributes: HashMap::new(),
            transferable: true,
            privacy_config: None,
        };
        
        let token_id = collection.mint(
            &mut TxContext::default(), 
            metadata, 
            None
        ).expect("Minting should succeed");
        
        // Attempt a transfer from an unauthorized address.
        let result = collection.transfer(
            &mut TxContext::default(), 
            token_id,
            &unauthorized, 
            &recipient, 
            Some(Amount::from(1_000_000u64))
        );
        assert!(matches!(result, Err(NftError::Unauthorized)));
    }
}
