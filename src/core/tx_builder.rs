use super::{Transaction, U256, Address};

#[derive(Debug, Default, Clone)]
pub struct TransactionBuilder {
	nonce: U256,
	gas_price: U256,
	gas: U256,
	sender: Address,
	mem_usage: usize,
}

impl TransactionBuilder {
	pub fn tx(&self) -> Self {
		self.clone()
	}

	pub fn nonce<T: Into<U256>>(mut self, nonce: T) -> Self {
		self.nonce = nonce.into();
		self
	}

	pub fn gas_price<T: Into<U256>>(mut self, gas_price: T) -> Self {
		self.gas_price = gas_price.into();
		self
	}

	pub fn sender<T: Into<Address>>(mut self, sender: T) -> Self {
		self.sender = sender.into();
		self
	}

	pub fn mem_usage(mut self, mem_usage: usize) -> Self {
		self.mem_usage = mem_usage;
		self
	}

	pub fn new(self) -> Transaction {
		let hash = self.nonce ^ (U256::from(100) * self.gas_price) ^ (U256::from(100_000) * U256::from(self.sender.low_u64()));
		Transaction {
			hash: hash.into(),
			nonce: self.nonce,
			gas_price: self.gas_price,
			gas: 21_000.into(),
			sender: self.sender,
			mem_usage: self.mem_usage,
		}
	}
}
