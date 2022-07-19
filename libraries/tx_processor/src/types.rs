use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct NormalizedTx {
	pub tx_hash: String,
	pub block_number: usize,
	pub value: usize,
	pub from: String,
	pub to: String,
	pub timestamp: usize,
	pub error: bool,
	pub fee: usize,
}

#[derive(Debug, Clone)]
pub struct TokenTransfer {
	pub tx_hash: String,
	pub block_number: usize,
	pub value: u64,
	pub from: String,
	pub to: String,
	pub token_name: String,
	pub authority: String,
	pub decimals: u8,
	pub mint_address: String,
	pub timestamp: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransferParams {
	pub from: String,
	pub to: String,
	pub value: String,
}
// pub struct TxnFeatures{
//   token_transfers: Vec<TokenTransfer>
// }
