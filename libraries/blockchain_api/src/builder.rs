use crate::{EvmApi, SolanaApi};
use common::{BlockchainName, BlockchainType};
pub struct BlockchainApiBuilder {
	rpc_url: Option<String>,
	ws_url: Option<String>,
	blockchain_name: Option<BlockchainName>,
}

pub enum BlockchainApiBuilded {
	Evm(EvmApi),
	Sol(SolanaApi),
}

impl BlockchainApiBuilder {
	pub fn new() -> BlockchainApiBuilder {
		BlockchainApiBuilder {
			rpc_url: None,
			ws_url: None,
			blockchain_name: None,
		}
	}
	pub fn set_rpc_url(mut self, rpc_url: String) -> BlockchainApiBuilder {
		self.rpc_url = Some(rpc_url);
		self
	}
	pub fn set_ws_url(mut self, ws_url: String) -> BlockchainApiBuilder {
		self.ws_url = Some(ws_url);
		self
	}

	pub fn set_blockchain_name(mut self, blockchain_name: BlockchainName) -> BlockchainApiBuilder {
		self.blockchain_name = Some(blockchain_name);
		self
	}

	pub fn build(&self) -> BlockchainApiBuilded {
		match self.blockchain_name.as_ref().unwrap().get_type() {
			BlockchainType::SOL => BlockchainApiBuilded::Sol(SolanaApi {
				rpc_url: self.rpc_url.clone().unwrap(),
				blockchain_name: self.blockchain_name.clone().unwrap(),
			}),
			BlockchainType::EVM => BlockchainApiBuilded::Evm(EvmApi {
				rpc_url: self.rpc_url.clone().unwrap(),
				ws_url: self.ws_url.clone().unwrap(),
				blockchain_name: self.blockchain_name.clone().unwrap(),
			}),
		}
	}
}
