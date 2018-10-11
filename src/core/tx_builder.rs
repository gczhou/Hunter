use super::{SignedTransaction, UnverifiedTransaction, Transaction};
use super::{U256, Address, RegionID};

#[derive(Debug, Default, Clone)]
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
			nonce: 0,
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
			action: self.action,
			data: self.data,
		}
    }

	pub fn set_region_id(&mut self, region_id: RegionID) -> Self {
		self.region_id = region_id;
		self
	}

	pub fn set_nonce(&mut self, nonce: U256) -> Self {
		self.nonce = nonce;
		self
	}

	pub fn set_block_limit(&mut self, block_limit: BlockNumber) -> Self {
		self.block_limit = block_limit;
		self
	}

	pub set_action(&mut self, action: Action) -> Self {
		self.action = action;
		self
	}

	pub set_data(&mut self, data: Bytes) -> Self {
		self.data = data;
		self
	}
}

pub struct UnverifiedTransactionBuilder {
	/// Plain Transaction.
    unsigned: Transaction,
    /// signature of the transaction
    seal: Bytes,
    /// Hash of the transaction
    hash: H256,
}

impl UnverifiedTransactionBuilder {
	pub fn build(tx: Transaction, seal: Bytes, hash: H256) -> UnverifiedTransaction {
		UnverifiedTransaction {
			unsigned: Transaction,
			seal: seal,
			hash: hash,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedTransactionBuilder {
    transaction: UnverifiedTransaction,
    sender: Address,
    public: Option<H512>,
}

impl SignedTransactionBuilder {
	pub fn build(tx: UnverifiedTransaction, sender: Address, public: Option<H512>) -> SignedTransaction {
		SignedTransaction {
			transaction: tx,
			sender: sender,
			public: public,
		}
	}
}
