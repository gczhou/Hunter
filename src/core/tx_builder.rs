use super::{SignedTransaction, UnverifiedTransaction, Transaction};
use super::{Hash256, Hash512, Uint256, Address, RegionID};
use super::BlockNumber;
use super::Action;
use super::Bytes;
use super::Secret;
use super::{H160, U256};

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    /// RegionID 
    region_id: RegionID,
    /// Nonce.
    nonce: Uint256,
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
			nonce: Uint256::default(),
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

	pub fn set_nonce(&mut self, nonce: Uint256) -> &mut Self {
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
	pub fn build(tx: Transaction, secret: &Secret) -> UnverifiedTransaction {
		let hash = tx.hash();
		println!("=============== Hash {:?}", hash);
		let sign = secret.key_pair.sign(&hash).expect("transaction signature fail.");
		//println!("Command Secret {:?} {:?}", secret.key_pair.get_privatekey_hex(), secret.key_pair.get_publickey());
		println!("Signature {:?}", &sign[..]);
		let utx = tx.with_rsv(Uint256::from(&sign[0..32]), Uint256::from(&sign[32..64]), sign[64] as u8);
		match utx.recover_public_and_sender() {
			Ok((public, sender)) => {
				println!("Public {:?} Sender {:?}", public, sender);
			},
			Err(err) => {
				println!("err: {:?}", err);
			},
		}
		utx
	}
}

pub struct SignedTransactionBuilder;
impl SignedTransactionBuilder {
	pub fn build(utx: UnverifiedTransaction) -> SignedTransaction {
		println!("UnverifiedTransaction {:?} Unsigned {:?}", utx, utx.deref());
		match SignedTransaction::new(utx) {
			Ok(st) => {
				st
			},
			Err(err) => {
				println!("Error info: {:?}", err);
				panic!("panic")
			}
		}
	}
}

/// Transaction test transaction deserialization.
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TransactionRequest {
    /// region_id
    #[serde(rename="regionid")]
    pub region_id: H160,
    /// Nonce.
    pub nonce: U256,
    /// Block limit.
    #[serde(rename="blocklimit")]
    pub block_limit: u64,
    pub from: H160,
    pub to: Option<H160>,
    /// data
    pub data: Bytes,
    /// R.
    pub r: U256,
    /// S.
    pub s: U256,
    /// V.
    pub v: U256,
}

impl TransactionRequest {
	pub fn new(tx: Transaction, secret: &Secret) -> Self {
		let hash = tx.hash();
		println!("=============== Hash {:?}", hash);
		let sign = secret.key_pair.sign(&hash).expect("transaction signature fail.");
		//println!("Command Secret {:?} {:?}", secret.key_pair.get_privatekey_hex(), secret.key_pair.get_publickey());
		println!("Signature {:?}", &sign[..]);
		let utx = tx.clone().with_rsv(Uint256::from(&sign[0..32]), Uint256::from(&sign[32..64]), sign[64] as u8);
        let st = SignedTransactionBuilder::build(utx);
		TransactionRequest {
			region_id: tx.region_id.into(),
			nonce: tx.nonce.into(),
			block_limit: tx.block_limit,
			from: st.sender().into(),
			to: match tx.action {
                Action::Create => None,
                Action::Call(address) => Some(address.into()),
            },
			data: tx.data,
			r: Uint256::from(&sign[0..32]).into(),
			s: Uint256::from(&sign[32..64]).into(),
			v: Uint256::from(&sign[64..]).into(),
		}
	}
}
