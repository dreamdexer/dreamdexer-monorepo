use crate::{RpcReqBody, RpcResponseBody};
use anyhow::Result;
use reqwest;
use serde::de::DeserializeOwned;
use tokio::time::Duration;

static MAX_TX_RECEIPT_PER_REQ: usize = 20;

pub struct RpcClient;

impl RpcClient {
	pub async fn send_request<T: DeserializeOwned>(rpc_url: &str, body: RpcReqBody) -> Result<RpcResponseBody<T>> {
		let client = reqwest::Client::new();

		let mut resp = client.post(rpc_url).json(&body).send().await;

		while resp.is_err() {
			tokio::time::sleep(Duration::from_millis(200)).await;
			resp = client.post(rpc_url).json(&body).send().await;
		}

		let deserialized_response = resp.unwrap().json::<RpcResponseBody<T>>().await?;
		Ok(deserialized_response)
	}

	pub async fn send_batch_request<T: DeserializeOwned + Clone>(
		rpc_url: &str,
		requests: Vec<RpcReqBody>,
		max_request_per_batch: Option<usize>,
	) -> Result<Vec<T>> {
		let client = reqwest::Client::new();
		let mut batch: Vec<&RpcReqBody> = vec![];
		let mut tx_receipts: Vec<T> = vec![];
		for (i, body) in requests.iter().enumerate() {
			batch.push(body);
			if (i % max_request_per_batch.unwrap_or(MAX_TX_RECEIPT_PER_REQ) == 0) || (i == requests.len() - 1) {
				let mut resp = client.post(rpc_url).json(&batch).send().await;
				while resp.is_err() {
					tokio::time::sleep(Duration::from_millis(1000)).await;
					resp = client.post(rpc_url).json(&requests).send().await;
				}

				let _ = &tx_receipts.append(
					&mut resp
						.unwrap()
						.json::<Vec<RpcResponseBody<T>>>()
						.await?
						.iter()
						.map(|elm| elm.result.clone())
						.collect::<Vec<T>>(),
				);
				batch.clear();
			}
		}
		Ok(tx_receipts)
	}
}
