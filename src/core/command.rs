use std::cmp::Ordering;

use super::Secret;
use super::*;

pub struct Command {
    config: Config,
}

impl Command {
    pub fn new(config: Config) -> Self {
        Command {
            config: config,
        }
    }
}

impl Command {
    pub fn send_transaction(&self, tx_str: String) {
        let mut secret = Secret::new(self.config.private_key);
        println!("private key {:?}", self.config.private_key);
        let transaction: SignedTransaction;
        if tx_str == "DEFAULT_TX".to_string() {
            let tx = TransactionBuilder::new().build();
            let utx = UnverifiedTransactionBuilder::build(tx, &mut secret);
            let st = SignedTransactionBuilder::build(utx);
            transaction = st;
            println!("transaction {:?}", transaction);
        } else {
            //
            //transaction = st;
        }
    }
}
