use super::AsymmetricKeyPair;
use super::H256;

pub struct Secret {
    pub key_pair: AsymmetricKeyPair,
}

impl Secret {
    pub fn new(private_key: H256) -> Self {
        let mut key_pair = AsymmetricKeyPair::new(Default::default());
        key_pair.from_private(&private_key);
        Secret {
            key_pair: key_pair,
        }
    }
}
