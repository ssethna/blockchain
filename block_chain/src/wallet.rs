//!
//! This is the Wallet module.<br>
//! This contains the Wallet and Transaction Struct.
//!

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

///
/// Struct Wallet.<br>
/// It includes address, keys, <br>
/// balance and transaction history.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
    pub balance: f64,
    pub transaction_history: VecDeque<Transaction>,
}

///
/// Struct Transaction.<br>
/// It includes tx_id, sender, <br>
/// receiver, amount and time stamp.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

///
/// Functions for Struct Wallet.
///
impl Wallet {
    pub fn new(address: String, private_key: String, public_key: String) -> Self {
        Wallet {
            address,
            private_key,
            public_key,
            balance: 0.0,
            transaction_history: VecDeque::new(),
        }
    }

    ///
    /// Process a new transaction.
    ///
    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transaction_history.push_back(tx);
    }

    ///
    /// Update wallet balance.
    ///
    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount;
    }

    ///
    /// Get wallet balance.
    ///
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    ///
    /// Get the transaction history of the wallet.
    ///
    pub fn get_transaction_history(&self) -> &VecDeque<Transaction> {
        &self.transaction_history
    }

    ///
    /// Load wallet data from database.
    ///
    pub fn load_from_file(filename: &str) -> Wallet {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    }
}
