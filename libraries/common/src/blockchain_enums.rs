use std::fmt::{Debug, Formatter, Result};



pub enum BlockchainType{
	SOL,
	EVM
}

#[derive(Clone)]
pub enum BlockchainName {
	SOLANA,
	ETHEREUM,
	AVALANCHE
}


impl BlockchainName {
    pub fn get_type(&self) -> BlockchainType {
        match self {
            BlockchainName::SOLANA => BlockchainType::SOL,
            BlockchainName::ETHEREUM => BlockchainType::EVM,
			BlockchainName::AVALANCHE => BlockchainType::EVM,

        }
    }
}
impl Debug for BlockchainName {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			BlockchainName::SOLANA => write!(f, "solana"),
			BlockchainName::ETHEREUM => write!(f, "ethereum"),
			BlockchainName::AVALANCHE => write!(f, "avalanche"),

		}
	}
}
