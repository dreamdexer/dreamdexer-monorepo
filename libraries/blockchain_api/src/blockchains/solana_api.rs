use crate::BlockchainApi;
use common::BlockchainName;
use common::{RpcReqBody, RpcResponseBody, SolanaTxn};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::value::Value;
use serde_json::{from_value, json};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SolanaApi {
	pub rpc_url: String,
	pub blockchain_name: BlockchainName,
}

#[async_trait]
impl BlockchainApi for SolanaApi {
	type TxnType = SolanaTxn;
	async fn get_latest_block(&self) -> Result<usize> {
		let client = reqwest::Client::new();
		let get_signatures_body = RpcReqBody {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "getBlockProduction".to_string(),
			params: None,
		};

		let resp = client
			.post(&self.rpc_url)
			.json(&get_signatures_body)
			.send()
			.await?
			.json::<HashMap<String, Value>>()
			.await?;

		let latest_block: usize = from_value::<usize>(resp["result"]["context"]["slot"].clone()).unwrap();
		Ok(latest_block)
	}

	async fn get_txn(&self, txn_hash: &String) -> Result<Self::TxnType> {
		let client = reqwest::Client::new();
		let get_txns_body: RpcReqBody = RpcReqBody {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "getTransaction".to_string(),
			params: Some(vec![
				serde_json::to_value(txn_hash).unwrap(),
				serde_json::to_value("json").unwrap(),
			]),
		};

		let resp = client.post(&self.rpc_url).json(&get_txns_body).send().await?;

		let decoded_response = resp.json::<RpcResponseBody<Self::TxnType>>().await?;
		Ok(decoded_response.result)
	}
}

impl SolanaApi {
	pub async fn get_signatures(&self, address: &str, from_block: u32, to_block: u32) -> Result<Vec<String>> {
		let mut last_signature: Option<String> = None;
		let mut all_signatures: Vec<Value> = vec![];
		let client = reqwest::Client::new();
		let signatures_limit = 1000;
		loop {
			let get_signatures_body = RpcReqBody {
				jsonrpc: "2.0".to_string(),
				id: 1,
				method: "getSignaturesForAddress".to_string(),
				params: Some(vec![
					serde_json::to_value(address).unwrap(),
					json!({
					  "limit":signatures_limit,
					  "before":last_signature
					}),
				]),
			};

			let resp = client
				.post(&self.rpc_url)
				.json(&get_signatures_body)
				.send()
				.await?
				.json::<HashMap<String, Value>>()
				.await?;

			let signatures_response: Vec<Value> = from_value(resp["result"].clone()).unwrap();

			let signatures_length = signatures_response.len();
			all_signatures.append(&mut signatures_response.clone());
			if from_value::<u32>(all_signatures[all_signatures.len() - 1]["slot"].clone()).unwrap() <= from_block {
				break;
			}
			if signatures_length < signatures_limit {
				break;
			}
			last_signature =
				Some(from_value::<String>(signatures_response[signatures_length - 1]["signature"].clone()).unwrap());
		}

		let mut result_signatures: Vec<String> = vec![];

		for elm in all_signatures.iter() {
			if from_value::<u32>(elm["slot"].clone()).unwrap() >= from_block
				&& from_value::<u32>(elm["slot"].clone()).unwrap() <= to_block
			{
				result_signatures.push(from_value::<String>(elm["signature"].clone()).unwrap());
			}
		}
		println!("Txs signatures {}", result_signatures.len());
		Ok(result_signatures)
	}

	pub async fn get_txns(&self, txn_hashes: &Vec<String>) -> Result<Vec<SolanaTxn>> {
		let client = reqwest::Client::new();
		let get_txns_body: Vec<RpcReqBody> = txn_hashes
			.iter()
			.map(|txn_hash| RpcReqBody {
				jsonrpc: "2.0".to_string(),
				id: 1,
				method: "getTransaction".to_string(),
				params: Some(vec![
					serde_json::to_value(txn_hash).unwrap(),
					serde_json::to_value("json").unwrap(),
				]),
			})
			.collect();

		let resp = client.post(&self.rpc_url).json(&get_txns_body).send().await?;

		let decoded_response = resp.json::<Vec<RpcResponseBody<SolanaTxn>>>().await?;

		let results: Vec<SolanaTxn> = decoded_response.iter().map(|elm| elm.result.clone()).collect();
		Ok(results)
	}

	pub async fn get_txns_by_address(&self, address: &str, from_block: u32, to_block: u32) -> Result<Vec<SolanaTxn>> {
		let address_signatures = self.get_signatures(address, from_block, to_block).await?;
		let response = self.get_txns(&address_signatures).await?;
		Ok(response)
	}
}
