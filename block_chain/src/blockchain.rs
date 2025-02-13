//!
//! This is the Blockchain module.<br>
//! The Bockchain struct is used to create blockchains.
//!

use crate::block::Block;
use crate::wallet::{Transaction, Wallet};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
        Blockchain { chain: Vec::new() }
    }

    ///
    /// Function to add a block to the chain.
    ///
    pub fn add_block(&mut self, data: Value) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.block_index + 1,
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

        // Convert the transaction to a Value type
        let transaction_data =
            serde_json::to_value(&transaction).expect("Failed to serialize transaction");

        self.add_block(transaction_data);
    }
}
