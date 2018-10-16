// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Lenient uint json deserialization for test json files.

use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use super::Uint256;

const LENGTH_0X : usize = 2;
const BITS_PER_BYTES : usize = 8;

macro_rules! impl_uint {
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
						/* 检测value的长度，应该为inner的长度一样 */
						let inner_length;
						if value.clone().starts_with("0x") {
							inner_length = $inner::max_value().bits() / BITS_PER_BYTES * 2 + LENGTH_0X;
						} else {
							inner_length = $inner::max_value().bits() / BITS_PER_BYTES * 2;
						}

						if value.len() > inner_length {
							return Err(Error::custom(format!("Invalid value {:?} length {}: {}", value, value.len(), inner_length)));
						}

						self.visit_str(value.as_ref())
					}
				}

				deserializer.deserialize_any(HashVisitor)
			}
		}

		impl Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
				let mut hex = "0x".to_owned();
				hex.push_str(&self.0.to_hex());
				serializer.serialize_str(&hex)
			}
		}
	}
}


impl_uint!(U256, Uint256);

impl Into<u64> for U256 {
	fn into(self) -> u64 {
		u64::from(self.0)
	}
}

impl Into<usize> for U256 {
	fn into(self) -> usize {
		// TODO: clean it after util conversions refactored.
		u64::from(self.0) as usize
	}
}

impl Into<u8> for U256 {
	fn into(self) -> u8 {
		u64::from(self.0) as u8
	}
}
