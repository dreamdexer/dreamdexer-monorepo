use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedTx {
	pub hash: String,
	pub block_number: i32,
	pub value: String,
	pub from: String,
	pub to: Option<String>,
	pub error: bool,
	pub fee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct NormalizedTokenTransfer {
	pub tx_hash: String,
	pub block_number: i32,
	pub value: String,
	pub from: String,
	pub to: String,
	pub token_ticker: String,
	pub token_name: String,
	pub token_address: String,
	pub decimals: u8,
}
