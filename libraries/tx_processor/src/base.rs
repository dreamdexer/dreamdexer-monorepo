use anyhow::Result;
use async_trait::async_trait;

use common::{NormalizedTokenTransfer, NormalizedTx};

#[async_trait]
pub trait TxnProcessor {
	type TxnType;
	fn get_token_transfers(txn: &Self::TxnType) -> Result<Vec<NormalizedTokenTransfer>>;
	fn normalize_txn(txn: &Self::TxnType) -> Result<NormalizedTx>;
	fn process_txns(&self, txsn: &Vec<Self::TxnType>) -> Result<()>;
}
