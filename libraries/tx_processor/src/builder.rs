use super::blockchains::SolanaTxnProcessor;
use common::BlockchainName;

pub struct TxnProcessorBuilder {
	blockchain_name: Option<BlockchainName>,
}

impl TxnProcessorBuilder {
	pub fn new() -> TxnProcessorBuilder {
		TxnProcessorBuilder { blockchain_name: None }
	}

	pub fn set_blockchain_name(mut self, blockchain_name: BlockchainName) -> TxnProcessorBuilder {
		self.blockchain_name = Some(blockchain_name);
		self
	}
	pub fn build(&self) -> SolanaTxnProcessor {
		match self.blockchain_name {
			Some(BlockchainName::SOLANA) => SolanaTxnProcessor {
				blockchain_name: self.blockchain_name.clone().unwrap(),
			},
			_ => SolanaTxnProcessor {
				blockchain_name: self.blockchain_name.clone().unwrap(),
			},
		}
	}
}
