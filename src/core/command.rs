use std::cmp::Ordering;

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
        let transaction: SignedTransaction;
        if tx_str == "DEFAULT_TX".to_string() {
            let tx = TransactionBuilder::new().build();
            let utx = UnverifiedTransactionBuilder::build(tx);
            let st = SignedTransactionBuilder::build(utx);
            transaction = st;
        } else {
            //
            //transaction = st;
        }
    }
}
