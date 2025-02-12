//!
//! Main runner for the Blockchain Application.<br>
//! Function main.<br>
//! List, transact and reset the blockchain sample application.
//!

use block_chain::blockchain::Blockchain;
use block_chain::wallet::{Transaction, Wallet};
use chrono::Utc;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, i16};
use uuid::Uuid;

///
/// Function main for the blockchain application.
///
fn main() {
    // Load blockchain from file
    let mut blockchain = Blockchain::new();
    if Path::new("./database/blockchain.json").exists() {
        blockchain = Blockchain::load_from_file("./database/blockchain.json");
        //blockchain.load_blocks_from_file("./database/blocks.json");
    }

    // Load wallets from files
    let mut wallet1 = Wallet::load_from_file("./database/wallet1.json");
    let mut wallet2 = Wallet::load_from_file("./database/wallet2.json");

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
                4 => send_crypto(&mut blockchain, &mut wallet1, &mut wallet2, 1),
                5 => send_crypto(&mut blockchain, &mut wallet2, &mut wallet1, 2),
                6 => reset_sample_data(&mut blockchain, &mut wallet1, &mut wallet2),
                7 => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice. Please enter a number between 1 and 6."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

///
/// List blockchain data.
///
fn list_blockchain(blockchain: &Blockchain) {
    println!("\n\nListing Blockchain...");

    for block in &blockchain.chain {
        //println!("{:?}\nblock data: {}\n", block, block.data);
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
    let pretty_wallet = serde_json::to_string_pretty(wallet).expect("Failed to pretty print block");
    println!("{}\n", pretty_wallet);
}

///
/// Crypto transaction from one wallet to another.
///
fn send_crypto(blockchain: &mut Blockchain, wallet1: &mut Wallet, wallet2: &mut Wallet, i: i16) {
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
        return;
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

    blockchain
        .add_block(serde_json::to_string(&transaction).expect("Failed to serialize transaction"));

    // Save updated blockchain and wallets to files
    fs::write(
        "./database/blocks.json",
        serde_json::to_string(&blockchain.chain).expect("Failed to serialize blocks"),
    )
    .expect("Unable to write blocks file");

    fs::write(
        "./database/blockchain.json",
        serde_json::to_string(&blockchain).expect("Failed to serialize blockchain"),
    )
    .expect("Unable to write blockchain file");

    if i == 1 {
        fs::write(
            "./database/wallet1.json",
            serde_json::to_string(&wallet1).expect("Failed to serialize wallet"),
        )
        .expect("Unable to write wallet1 file");

        fs::write(
            "./database/wallet2.json",
            serde_json::to_string(&wallet2).expect("Failed to serialize wallet"),
        )
        .expect("Unable to write wallet2 file");
    } else {
        fs::write(
            "./database/wallet1.json",
            serde_json::to_string(&wallet2).expect("Failed to serialize wallet"),
        )
        .expect("Unable to write wallet1 file");

        fs::write(
            "./database/wallet2.json",
            serde_json::to_string(&wallet1).expect("Failed to serialize wallet"),
        )
        .expect("Unable to write wallet2 file");
    }
    println!("Transaction successful, blockchain and wallets updated\n");
}

///
/// Reset application data to start over.
///
fn reset_sample_data(blockchain: &mut Blockchain, wallet1: &mut Wallet, wallet2: &mut Wallet) {
    println!("Resetting Sample Data...");

    let source_files = [
        ("./database/bkup_blocks.json", "./database/blocks.json"),
        (
            "./database/bkup_blockchain.json",
            "./database/blockchain.json",
        ),
        (
            "./database/bkup_transactions.json",
            "./database/transactions.json",
        ),
        ("./database/bkup_wallet1.json", "./database/wallet1.json"),
        ("./database/bkup_wallet2.json", "./database/wallet2.json"),
    ];

    for (source, destination) in source_files.iter() {
        match fs::copy(source, destination) {
            Ok(_) => println!("Successfully copied {} to {}", source, destination),
            Err(e) => println!("Failed to copy {} to {}: {}", source, destination, e),
        }
    }

    *blockchain = Blockchain::new();
    println!("Blockchain has been reset to its initial state.\n");

    // Reload wallets from reset files
    *wallet1 = Wallet::load_from_file("./database/wallet1.json");
    *wallet2 = Wallet::load_from_file("./database/wallet2.json");
    println!("Wallets have been reset to their initial state.\n");
}
