//!
//! This is the MongoDB_Utils module.<br>
//! To do CRUD operations for the blockcahin.
//!

use block_chain::block::Block;
use block_chain::blockchain::Blockchain;
use block_chain::wallet::Wallet;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document},
    Collection,
};
use std::error::Error;

///
/// Functions Reset Sample Data.
///
pub async fn reset_sample_data(
    blockchain_collection: Collection<Document>,
    blocks_collection: Collection<Document>,
    wallets_collection: Collection<Document>,
) -> Result<(), Box<dyn Error>> {
    println!("Resetting Sample Data...");

    // Clear existing blockchain
    blockchain_collection.delete_many(doc! {}).await?;

    // Clear existing blocks
    blocks_collection.delete_many(doc! {}).await?;

    // Clear existing wallets
    wallets_collection.delete_many(doc! {}).await?;

    // Sample data for blockchain and wallets
    let genesis_blockchain = doc! {
        "id": 1,
        "chain": [
            {
                "block_index": 0,
                "timestamp": 0,
                "previous_hash": "0",
                "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "data": "Genesis Block"
            }
        ]
    };

    let genesis_block = doc! {
        "block_index": 0,
        "timestamp": 0,
        "previous_hash": "0",
        "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "data": "Genesis Block"
    };

    let wallet1_data = doc! {
        "address": "address1",
        "private_key": "private_key1",
        "public_key": "public_key1",
        "balance": 50.0,
        "transaction_history": []
    };

    let wallet2_data = doc! {
        "address": "address2",
        "private_key": "private_key2",
        "public_key": "public_key2",
        "balance": 20.0,
        "transaction_history": []
    };

    // Insert genesis blockchain
    blockchain_collection.insert_one(genesis_blockchain).await?;

    // Insert genesis block
    blocks_collection.insert_one(genesis_block).await?;

    // Insert new wallets
    wallets_collection.insert_one(wallet1_data).await?;
    wallets_collection.insert_one(wallet2_data).await?;

    println!("Sample data has been reset.\n");

    Ok(())
}

///
/// Functions to Read the Blockchain.
///
pub async fn load_blockchain(
    blocks_collection: Collection<Document>,
    blockchain_collection: Collection<Document>,
) -> Result<Blockchain, Box<dyn Error>> {
    let mut blockchain = Blockchain::new();

    // Load from `blocks` collection
    let mut cursor = blocks_collection.find(doc! {}).await?;
    println!("\nLoading blocks from MongoDB:");
    while let Some(result) = cursor.try_next().await? {
        let block: Block = bson::from_document(result)?;
        let pretty_block =
            serde_json::to_string_pretty(&block).expect("Failed to pretty print block");
        println!("{}", pretty_block);
        blockchain.chain.push(block);
    }

    // Load from `blockchain` collection
    let blockchain_doc = blockchain_collection.find_one(doc! { "id": 1 }).await?;
    if let Some(doc) = blockchain_doc {
        let chain: Vec<Block> =
            bson::from_bson(doc.get("chain").unwrap().clone()).unwrap_or_default();
        for block in chain {
            if !blockchain.chain.contains(&block) {
                blockchain.chain.push(block);
            }
        }
    }

    println!("\nFinal blockchain length: {}\n\n", blockchain.chain.len());
    Ok(blockchain)
}

///
/// Functions to Read Wallet.
///
pub async fn load_wallet(
    wallets_collection: Collection<Document>,
    address: &str,
) -> Result<Wallet, Box<dyn Error>> {
    let filter = doc! {"address": address};
    let result = wallets_collection.find_one(filter).await?;

    if let Some(doc) = result {
        let wallet: Wallet = bson::from_document(doc)?;
        Ok(wallet)
    } else {
        Err("Wallet not found".into())
    }
}

///
/// Functions to Update Blockchain.
///
pub async fn save_blockchain(
    blockchain: &Blockchain,
    blocks_collection: Collection<Document>,
    blockchain_collection: Collection<Document>,
) -> Result<(), Box<dyn Error>> {
    // Drop and save individual blocks
    blocks_collection.drop().await?;
    for block in &blockchain.chain {
        let doc = bson::to_document(block)?;
        blocks_collection.insert_one(doc).await?;
    }

    // Save the entire blockchain
    let blockchain_doc = doc! {
        "id": 1,
        "chain": bson::to_bson(&blockchain.chain)?
    };
    blockchain_collection
        .update_one(doc! { "id": 1 }, doc! { "$set": blockchain_doc })
        .await?;

    Ok(())
}

///
/// Functions to Update Wallet.
///
pub async fn save_wallet(
    wallet: Wallet,
    wallets_collection: Collection<Document>,
) -> Result<(), Box<dyn Error>> {
    let filter = doc! {"address": wallet.address.clone()};
    let update = bson::to_document(&wallet)?;
    wallets_collection.replace_one(filter, update).await?;
    Ok(())
}
