//!
//! Main runner for the Blockchain Application.<br>
//! Function main.<br>
//! List, transact and reset the blockchain sample application.
//!

mod mongo_utils;

use block_chain::block::Block;
use block_chain::blockchain::Blockchain;
use block_chain::wallet::{Transaction, Wallet};
use bson::Document;
use chrono::Utc;
use mongodb::{Client, Collection};
use std::error::Error;
use std::io::{self, Write};
use tokio;
use uuid::Uuid;

///
/// Function main for the blockchain application.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize MongoDB client
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("blockchain_db");

    // Initialize collections
    let blocks_collection = db.collection("blocks");
    let wallets_collection = db.collection("wallets");
    let blockchain_collection = db.collection("blockchain");

    // Load blockchain from database
    let mut blockchain =
        mongo_utils::load_blockchain(blocks_collection.clone(), blockchain_collection.clone())
            .await?;

    // Load wallets from database
    let mut wallet1 = mongo_utils::load_wallet(wallets_collection.clone(), "address1").await?;
    let mut wallet2 = mongo_utils::load_wallet(wallets_collection.clone(), "address2").await?;

    loop {
        // Display the menu options
        println!("Please input your choice:");
        println!("1. List Blockchain");
        println!("2. View Wallet1");
        println!("3. View Wallet2");
        println!("4. Send Crypto from Wallet1 to Wallet2");
        println!("5. Send Crypto from Wallet2 to Wallet1");
        println!("6. Reset Sample Data");
        println!("7. Exit");
        print!("Input Choice: ");

        // Flush stdout to ensure the prompt is displayed
        io::stdout().flush().unwrap();

        // Read the user's input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim and parse the input
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(choice) => match choice {
                1 => list_blockchain(&blockchain),
                2 => view_wallet(&wallet1, "Wallet1"),
                3 => view_wallet(&wallet2, "Wallet2"),
                4 => {
                    send_crypto(
                        &mut blockchain,
                        &mut wallet1,
                        &mut wallet2,
                        1,
                        blocks_collection.clone(),
                        wallets_collection.clone(),
                        blockchain_collection.clone(),
                    )
                    .await?
                }
                5 => {
                    send_crypto(
                        &mut blockchain,
                        &mut wallet2,
                        &mut wallet1,
                        2,
                        blocks_collection.clone(),
                        wallets_collection.clone(),
                        blockchain_collection.clone(),
                    )
                    .await?
                }
                6 => {
                    mongo_utils::reset_sample_data(
                        blockchain_collection.clone(),
                        blocks_collection.clone(),
                        wallets_collection.clone(),
                    )
                    .await?;
                    blockchain = mongo_utils::load_blockchain(
                        blocks_collection.clone(),
                        blockchain_collection.clone(),
                    )
                    .await?;
                    wallet1 =
                        mongo_utils::load_wallet(wallets_collection.clone(), "address1").await?;
                    wallet2 =
                        mongo_utils::load_wallet(wallets_collection.clone(), "address2").await?
                }
                7 => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice. Please enter a number between 1 and 7."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }

    Ok(())
}

///
/// List blockchain data.
///
fn list_blockchain(blockchain: &Blockchain) {
    println!("\n\nListing Blockchain...");

    for block in &blockchain.chain {
        // Pretty print the data field if it is valid JSON
        let pretty_block =
            serde_json::to_string_pretty(block).expect("Failed to pretty print block");
        println!("{}\n", pretty_block);
    }

    println!("Is blockchain valid? {}\n\n", blockchain.is_chain_valid());
}

///
/// View wallet data.
///
fn view_wallet(wallet: &Wallet, wallet_name: &str) {
    println!("\n\nViewing {}...", wallet_name);
    let pretty_wallet =
        serde_json::to_string_pretty(wallet).expect("Failed to pretty print wallet");
    println!("{}\n", pretty_wallet);
}

///
/// Crypto transaction from one wallet to another.
///
async fn send_crypto(
    blockchain: &mut Blockchain,
    wallet1: &mut Wallet,
    wallet2: &mut Wallet,
    i: i16,
    blocks_collection: Collection<Document>,
    wallets_collection: Collection<Document>,
    blockchain_collection: Collection<Document>,
) -> Result<(), Box<dyn Error>> {
    if i == 1 {
        println!("\n\nSending Crypto from Wallet1 to Wallet2...");
    } else {
        println!("\n\nSending Crypto from Wallet2 to Wallet1...");
    }

    // Prompt the user to input the transaction amount
    print!("Please enter the amount to send: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let amount: f64 = input
        .trim()
        .parse()
        .expect("Invalid input, please enter a valid number");

    if wallet1.get_balance() < amount {
        println!("Transaction failed: Insufficient balance\n");
        return Ok(());
    }

    // Process transaction
    wallet1.update_balance(-amount);
    wallet2.update_balance(amount);

    let transaction = Transaction {
        tx_id: Uuid::new_v4().to_string(),
        sender: wallet1.address.clone(),
        receiver: wallet2.address.clone(),
        amount,
        timestamp: Utc::now().timestamp() as u64,
    };

    wallet1.add_transaction(transaction.clone());
    wallet2.add_transaction(transaction.clone());

    // Store the transaction directly as a JSON object
    let transaction_data =
        serde_json::to_value(&transaction).expect("Failed to serialize transaction");

    // Update Block struct to accept transaction_data as Value
    let block = Block::new(
        blockchain.chain.len() as u64,
        Utc::now().timestamp() as u64,
        blockchain
            .chain
            .last()
            .map_or(String::from("0"), |block| block.hash.clone()),
        transaction_data,
    );

    blockchain.chain.push(block);

    // Save updated blockchain and wallets to MongoDB
    mongo_utils::save_blockchain(blockchain, blocks_collection, blockchain_collection).await?;
    mongo_utils::save_wallet(wallet1.clone(), wallets_collection.clone()).await?;
    mongo_utils::save_wallet(wallet2.clone(), wallets_collection).await?;

    println!("Transaction successful, blockchain and wallets updated\n");

    Ok(())
}
