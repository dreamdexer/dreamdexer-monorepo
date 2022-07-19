use super::schema::txns;

#[derive(Insertable)]
#[table_name = "txns"]
pub struct NewTxn {
	pub hash: String,
	pub block_number: i32,
	pub value: String,
	pub from_address: String,
	pub to_address: Option<String>,
	pub error: bool,
	pub fee: String,
}

#[derive(Queryable, Debug)]
pub struct Txn {
	pub hash: String,
	pub block_number: i32,
	pub from_address: String,
	pub to_address: Option<String>,
	pub value: String,
	pub error: bool,
	pub fee: String,
}

#[derive(Queryable)]
pub struct TokenTransfer {
	pub id: u32,
	pub tx_hash: String,
	pub block_number: String,
	pub value: String,
	pub from_address: String,
	pub to_address: String,
	pub token_name: String,
	pub token_address: String,
	pub decimals: u8,
}

#[derive(Queryable)]
pub struct AssetTransfer {
	pub id: u32,
	pub tx_hash: String,
	pub block_number: String,
	pub value: String,
	pub from_address: String,
	pub to_address: String,
	pub asset_name: String,
	pub asset_address: String,
	pub decimals: u8,
	pub error: bool,
	pub tx_fee: String,
}
