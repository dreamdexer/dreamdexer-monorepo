extern crate dotenv;
mod producer;

use anyhow::Result;
use blockchain_api::EvmApi;
use blockchain_api::{BlockchainApiBuilded, BlockchainApiBuilder};
use common::BlockchainName;
use common::EvmCompleteTxn;
use dotenv::dotenv;
use futures::future::join_all;
use futures::Future;
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel};
use log::info;
use serde_json;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();

	//Build api

	let evm_api = BlockchainApiBuilder::new()
		.set_blockchain_name(BlockchainName::AVALANCHE)
		.set_rpc_url(String::from("https://api.avax.network/ext/bc/C/rpc"))
		.set_ws_url(String::from("wss://api.avax.network/ext/bc/C/ws"))
		.build();

	// Init rabbitmq client

	let producer = producer::Producer::connect(std::env::var("AMQP_ADDR").unwrap().to_string()).await;

	match evm_api {
		BlockchainApiBuilded::Evm(api) => {
			// let block_number = api.get_latest_block().await?;
			// println!("{:#?}", api.get_block(block_number).await?);
			let api_clone = api.clone();
			let channel = producer.channel.clone();
			let exchange_name = producer.exchange_name.clone();
			let start_block = 0;
			let end_block = 17000000;
			let mut futures_vec = vec![];

			for x in start_block..end_block {
				futures_vec.push(fetch_data(&channel, &exchange_name, x, &api_clone));
			}

			for a in start_block..end_block {
				if a % 40 == 0 || a == end_block {
					//TODO: join_all(futures_vec[a - 40..a]).await;
				}
			}
		}
		_ => {
			println!("Couldn't build your expected api")
		}
	}
	Ok(())
}

async fn fetch_data(channel: &Channel, exchange_name: &String, block_number: usize, api: &EvmApi) {
	{
		let block_data = api.get_block(block_number).await;
		let mut complete_txns: Vec<EvmCompleteTxn> = vec![];

		let transactions = block_data.as_ref().unwrap().transactions.clone();
		let txn_receipts = api
			.get_txn_receipt(transactions.iter().map(|elm| elm.hash.clone()).collect(), None)
			.await
			.unwrap();

		for x in 0..transactions.len() {
			complete_txns.push(EvmCompleteTxn {
				txn: transactions[x].clone(),
				txn_receipt: txn_receipts[x].clone(),
			})
		}

		let without_prefix = block_data.as_ref().unwrap().number.trim_start_matches("0x");
		let result: usize = usize::from_str_radix(without_prefix, 16).unwrap();
		info!("Number of transactions for block {}:  {}", result, transactions.len());
		let tx_receipt_str = serde_json::to_string::<Vec<EvmCompleteTxn>>(&complete_txns);
		let _published = channel
			.basic_publish(
				&exchange_name,
				"",
				BasicPublishOptions {
					mandatory: true,
					immediate: false,
				},
				tx_receipt_str.as_ref().unwrap().as_bytes(),
				BasicProperties::default(),
			)
			.await;
	}
}
