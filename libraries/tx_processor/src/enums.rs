use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub enum BlockchainName {
	SOLANA,
	ETHEREUM,
}

impl Debug for BlockchainName {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			BlockchainName::SOLANA => write!(f, "solana"),
			BlockchainName::ETHEREUM => write!(f, "ethereum"),
		}
	}
}
