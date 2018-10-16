
//! Lenient hash json deserialization for test json files.

use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use rustc_hex::ToHex;
use super::{H64 as Hash64, H160 as Hash160, H256 as Hash256, H512 as Hash512, H520 as Hash520, H2048 as Hash2048};

macro_rules! impl_hash {
	($name: ident, $inner: ident) => {
		/// Lenient hash json deserialization for test json files.
		#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
		pub struct $name(pub $inner);

		impl From<$name> for $inner {
			fn from(other: $name) -> $inner {
				other.0
			}
		}

		impl From<$inner> for $name {
			fn from(i: $inner) -> Self {
				$name(i)
			}
		}

		impl<'a> Deserialize<'a> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where D: Deserializer<'a> {

				struct HashVisitor;

				impl<'b> Visitor<'b> for HashVisitor {
					type Value = $name;

					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
						write!(formatter, "a 0x-prefixed hex-encoded hash")
					}

					fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
						let value = match value.len() {
							0 => $inner::from(0),
							2 if value == "0x" => $inner::from(0),
							_ if value.starts_with("0x") => $inner::from_str(&value[2..]).map_err(|e| {
								Error::custom(format!("Invalid hex value {}: {}", value, e).as_str())
							})?,
							_ => $inner::from_str(value).map_err(|e| {
								Error::custom(format!("Invalid hex value {}: {}", value, e).as_str())
							})?,
						};

						Ok($name(value))
					}

					fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
						self.visit_str(value.as_ref())
					}
				}

				deserializer.deserialize_any(HashVisitor)
			}
		}

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
            where
                S: Serializer,
            {
                let mut hex = "0x".to_owned();
                let self_str: &String = &self.0.to_hex();
                let length = self_str.len();
                // 如果为16长度，即u64或者h64的情况下，需要去掉前缀的零
                if length == 16 {
                    hex.push_str(&skip_prefix_0(&self_str));
                } else {
                    hex.push_str(&self_str);
                }
                serializer.serialize_str(&hex)
            }
        }
	}
}

fn skip_prefix_0(a: &str) -> &str {
    let length = a.len();

    let mut index = 0;
    for i in 0..length {
        let b = a.as_bytes()[i];
        if b.ne(&b'0') {
            index = i;
            break;
        }
        // 如果为全0的情况，保留一位0
        if i.eq(&(length-1)) {
            index = length -1;
        }
    }

    &a[index..]
}

impl_hash!(H64, Hash64);
impl_hash!(Address, Hash160);
impl_hash!(H160, Hash160);
impl_hash!(H256, Hash256);
impl_hash!(H512, Hash512);
impl_hash!(H520, Hash520);
impl_hash!(Bloom, Hash2048);