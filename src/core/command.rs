use std::cmp::Ordering;

use super::Secret;
use super::Config;
use super::Utils;
use super::{SignedTransaction};
use super::{UnverifiedTransactionBuilder, SignedTransactionBuilder, TransactionBuilder, TransactionRequest};
use super::Uint256;
use super::serde_json;

const BLOCK_LIMIT: u64 = 1000;

pub struct Command {
    config: Config,
    secret: Secret,
}

impl Command {
    pub fn new(config: Config) -> Self {
        let secret = Secret::new(config.private_key);
        println!("Command Secret {:?} {:?}", secret.key_pair.get_privatekey_hex(), secret.key_pair.get_publickey());
        Command {
            config: config,
            secret: secret,
        }
    }
}

impl Command {
    pub fn send_transaction(&self, tx_str: String) {
        if tx_str == "DEFAULT_TX".to_string() {
            let block_number = Utils::get_block_number();
            let tx = TransactionBuilder::new()
                                    .set_block_limit(block_number + BLOCK_LIMIT)
                                    .set_nonce(Uint256::default())
                                    .build();

            let transaction_request = TransactionRequest::new(tx, &self.secret);
            //let utx = UnverifiedTransactionBuilder::build(tx, &self.secret);

            //let st = SignedTransactionBuilder::build(utx);
            let request = serde_json::to_string(&transaction_request).unwrap();

            //transaction = st;
            println!("transaction {:?}", request);
        } else {
            //
            //transaction = st;
        }
    }
}
