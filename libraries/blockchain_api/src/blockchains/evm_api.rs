use crate::BlockchainApi;
use anyhow::Result;
use async_trait::async_trait;
use common::{BlockchainName, EvmBlock, EvmHead, EvmTxnReceipt, RpcClient, WebSocketResponseMessage};
use common::{EvmTxn, RpcReqBody};
use log::info;
use serde_json::from_str;
use std::result::Result::Ok;
use std::sync::mpsc::Sender;
use websocket::{ClientBuilder, OwnedMessage};

#[derive(Debug, Clone)]
pub struct EvmApi {
	pub rpc_url: String,
	pub ws_url: String,
	pub blockchain_name: BlockchainName,
}

#[async_trait]
impl BlockchainApi for EvmApi {
	type TxnType = EvmTxn;
	async fn get_latest_block(&self) -> Result<usize> {
		let get_latest_block_body = RpcReqBody {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "eth_blockNumber".to_string(),
			params: None,
		};

		let latest_block = RpcClient::send_request::<String>(&self.rpc_url, get_latest_block_body)
			.await
			.expect(&format!("Error reading block data from latest block"));

		let without_prefix = latest_block.result.trim_start_matches("0x");
		let result: usize = usize::from_str_radix(without_prefix, 16).unwrap();
		Ok(result)
	}

	async fn get_txn(&self, txn_hash: &String) -> Result<Self::TxnType> {
		let get_txns_body: RpcReqBody = RpcReqBody {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "eth_getTransactionByHash".to_string(),
			params: Some(vec![serde_json::from_str(txn_hash).unwrap()]),
		};

		let txn = RpcClient::send_request::<Self::TxnType>(&self.rpc_url, get_txns_body)
			.await
			.expect(&format!("Error reading tx data from tx_hash: {txn_hash}latest block"));

		Ok(txn.result)
	}
}

impl EvmApi {
	pub async fn get_block(&self, block_number: usize) -> Result<EvmBlock> {
		let block_number_hex = [String::from("0x"), format!("{:X}", block_number)].concat();
		let get_block_body = RpcReqBody {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "eth_getBlockByNumber".to_string(),
			params: Some(vec![
				serde_json::Value::String(block_number_hex),
				serde_json::Value::Bool(true),
			]),
		};

		let block = RpcClient::send_request::<EvmBlock>(&self.rpc_url, get_block_body)
			.await
			.expect(&format!("Error reading block data on block {}", block_number));
		Ok(block.result)
	}

	pub async fn get_txn_receipt(
		&self,
		tx_hashes: Vec<String>,
		max_request_per_batch: Option<usize>,
	) -> Result<Vec<EvmTxnReceipt>> {
		let get_txn_receipt_body = tx_hashes
			.iter()
			.map(|elm| RpcReqBody {
				jsonrpc: "2.0".to_string(),
				id: 1,
				method: "eth_getTransactionReceipt".to_string(),
				params: Some(vec![serde_json::Value::String(elm.clone())]),
			})
			.collect::<Vec<RpcReqBody>>();

		let tx_receipts =
			RpcClient::send_batch_request::<EvmTxnReceipt>(&self.rpc_url, get_txn_receipt_body, max_request_per_batch)
				.await
				.expect(&format!("Error reading tx receipts[{:#?}]", tx_hashes));
		Ok(tx_receipts)
	}

	pub fn suscribe_new_head(self: &Self, sender: &Sender<usize>) -> () {
		let mut client = ClientBuilder::new(&self.ws_url).unwrap().connect(None).unwrap();

		let message = websocket::Message::text(
			r#"{"jsonrpc":  "2.0",  "id":  1,  "method":  "eth_subscribe",  "params":  ["newHeads"]}"#,
		);
		client.send_message(&message).unwrap();

		for message in client.incoming_messages() {
			match message.unwrap() {
				OwnedMessage::Text(text) => {
					let block = from_str::<WebSocketResponseMessage<EvmHead>>(&text);
					match block {
						Ok(ref value) => {
							info!("New block was mined {}", value.params.result.hash);
							let without_prefix = value.params.result.number.trim_start_matches("0x");
							let result: usize = usize::from_str_radix(without_prefix, 16).unwrap();
							sender.send(result).unwrap();
						}
						Err(err) => {
							println!("Error reading block head: {} - {}", text, err);
						}
					}
				}
				_ => {
					println!("Arrived invalid message");
				}
			}
		}
	}
}
