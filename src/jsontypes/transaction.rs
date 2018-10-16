use super::uint::U256;
use super::bytes::Bytes;
use super::hash::Address;
use super::maybe::MaybeEmpty;
use super::{Transaction as BlockChainTransaction, UnverifiedTransaction as UTX, Action};

/// Transaction test transaction deserialization.
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Transaction {
    /// data
    pub data: Bytes,
    /// Nonce.
    pub nonce: U256,
    /// Block limit.
    #[serde(rename="blocklimit")]
    pub block_limit: U256,
    /// Value. must be zero
    pub value: Option<U256>,
    /// R.
    pub r: U256,
    /// S.
    pub s: U256,
    /// V.
    pub v: U256,
}

impl From<Transaction> for UTX {
    fn from(t: Transaction) -> Self {
        let to: Option<Address> = t.to.into();
        let tx = BlockChainTransaction {
            nonce: t.nonce.into(),
            action: match to {
                Some(t) => Action::Call(t.into()),
                None => Action::Create
            },
            data: t.data.into(),
            block_limit: t.block_limit.into(),
        };

        tx.with_rsv(t.r.into(), t.s.into(), t.v.into())
    }
}
