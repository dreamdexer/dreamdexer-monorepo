use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BlockchainApi {
	type TxnType;
	async fn get_latest_block(&self) -> Result<usize>;
	async fn get_txn(&self, tx_hash: &String) -> Result<Self::TxnType>;
}
