//!
//! This is the Blockchain module.<br>
//! The Bockchain struct is used to create blockchains.
//!

use crate::block::Block;
use crate::wallet::{Transaction, Wallet};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

///
/// Struct Blockchain.<br>
/// Holds a Vector of type Strut Block.
///
#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

///
/// Functions for Struct Blockchain.
///
impl Blockchain {
    ///
    /// Creates a new Blockchain
    ///
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::new(
                0,
                0,
                String::from("0"),
                String::from("Genesis Block"),
            )],
        }
    }

    ///
    /// Function to add a block to the chain.
    ///
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            chrono::Utc::now().timestamp() as u64,
            previous_block.hash.clone(),
            data,
        );
        self.chain.push(new_block);
    }

    ///
    /// Function to check if chain is valid.
    ///
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    ///
    /// Initial load of blockchain from database.
    ///
    pub fn load_from_file(filename: &str) -> Blockchain {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    }

    // pub fn load_blocks_from_file(&mut self, filename: &str) {
    //     let data = std::fs::read_to_string(filename).expect("Unable to read file");
    //     let blocks: Vec<Block> = serde_json::from_str(&data).expect("Unable to parse JSON");
    //     self.chain = blocks;
    // }

    ///
    /// Process a new transaction onto the blockchain.
    ///
    pub fn process_transaction(
        &mut self,
        sender_wallet: &mut Wallet,
        receiver_wallet: &mut Wallet,
        amount: f64,
    ) {
        if sender_wallet.get_balance() < amount {
            println!("Transaction failed: Insufficient balance");
            return;
        }

        sender_wallet.update_balance(-amount);
        receiver_wallet.update_balance(amount);

        let transaction = Transaction {
            tx_id: Uuid::new_v4().to_string(),
            sender: sender_wallet.address.clone(),
            receiver: receiver_wallet.address.clone(),
            amount,
            timestamp: Utc::now().timestamp() as u64,
        };

        sender_wallet.add_transaction(transaction.clone());
        receiver_wallet.add_transaction(transaction.clone());

        self.add_block(
            serde_json::to_string(&transaction).expect("Failed to serialize transaction"),
        );
    }

    ///
    /// Optional processing of transactions from a file.<br>
    /// Currently not used.
    ///
    pub fn load_transactions_from_file(&mut self, filename: &str, wallets: &mut [Wallet]) {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        let transactions: Vec<Transaction> =
            serde_json::from_str(&data).expect("Unable to parse JSON");

        for transaction in transactions {
            let sender_wallet_index = wallets.iter().position(|w| w.address == transaction.sender);
            let receiver_wallet_index = wallets
                .iter()
                .position(|w| w.address == transaction.receiver);

            if let (Some(sender_index), Some(receiver_index)) =
                (sender_wallet_index, receiver_wallet_index)
            {
                let (sender_wallet, receiver_wallet) = if sender_index < receiver_index {
                    let (left, right) = wallets.split_at_mut(receiver_index);
                    (&mut left[sender_index], &mut right[0])
                } else {
                    let (left, right) = wallets.split_at_mut(sender_index);
                    (&mut right[0], &mut left[receiver_index])
                };

                self.process_transaction(sender_wallet, receiver_wallet, transaction.amount);
            }
        }
    }
}
