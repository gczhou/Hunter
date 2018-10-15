use super::{SignedTransaction, UnverifiedTransaction, Transaction};
use super::{H256, H512, U256, Address, RegionID};
use super::BlockNumber;
use super::Action;
use super::Bytes;
use super::Secret;

#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    /// RegionID 
    region_id: RegionID,
    /// Nonce.
    nonce: U256,
    /// Block limit.
    block_limit: BlockNumber,
    /// Action, can be either call or contract create.
    action: Action,
    /// Transaction data.
    data: Bytes,
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder {
			region_id: RegionID::default(),
			nonce: U256::default(),
			block_limit: 1000,
			action: Action::default(),
			data: vec![],
        }
    }
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Transaction {
		Transaction {
			region_id: self.region_id,
			nonce: self.nonce,
			block_limit: self.block_limit,
			action: self.action.clone(),
			data: self.data.clone(),
		}
    }

	pub fn set_region_id(&mut self, region_id: RegionID) -> &mut Self {
		self.region_id = region_id;
		self
	}

	pub fn set_nonce(&mut self, nonce: U256) -> &mut Self {
		self.nonce = nonce;
		self
	}

	pub fn set_block_limit(&mut self, block_limit: BlockNumber) -> &mut Self {
		self.block_limit = block_limit;
		self
	}

	pub fn set_action(&mut self, action: Action) -> &mut Self {
		self.action = action;
		self
	}

	pub fn set_data(&mut self, data: Bytes) -> &mut Self {
		self.data = data;
		self
	}
}

pub struct UnverifiedTransactionBuilder;
impl UnverifiedTransactionBuilder {
	pub fn build(tx: Transaction, secret: &mut Secret) -> UnverifiedTransaction {
		let hash = tx.hash();
		let sign = secret.key_pair.sign(&hash).expect("Transaction signature fail.");
		println!("Key Pair {:?}", &sign[..]);
		let utx = tx.with_rsv(U256::from(&sign[0..31]), U256::from(&sign[32..63]), sign[64]);
		utx
	}
}

pub struct SignedTransactionBuilder;
impl SignedTransactionBuilder {
	pub fn build(utx: UnverifiedTransaction) -> SignedTransaction {
		SignedTransaction::new(utx).unwrap()
	}
}
