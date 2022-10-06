use common::NormalizedTx;
use common::{EvmCompleteTxn, NormalizedTokenTransfer};
use futures::future::join;
use futures::StreamExt;
use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use log::{error, info};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::str;
use std::sync::Mutex;
use tx_processor::EvmTxnProcessor;
use tx_processor::TxnProcessor;

pub struct ProcessorPipeline {
	pub connection: Connection,
	pub consume_channel: Channel,
	pub publish_txns_channel: Channel,
	pub publish_token_transfers_channel: Channel,
	pub rabbitmq_url: String,
	pub consume_queue_name: String,
	pub normalized_txns_queue_name: String,
	pub normalized_token_transfers_queue_name: String,
}

impl ProcessorPipeline {
	pub async fn connect(
		url: String,
		consume_queue_name: &str,
		normalized_txns_queue_name: &str,
		normalized_token_transfers_queue_name: &str,
	) -> Self {
		let conn = Connection::connect(&url, ConnectionProperties::default()).await;
		if conn.is_err() {
			error!(target: "consumer","RabbitMQ connection error")
		}
		let consume_channel = conn.as_ref().unwrap().create_channel().await;
		if consume_channel.is_err() {
			error!(target: "consumer","RabbitMQ channel error")
		}
		let publish_txns_channel = conn.as_ref().unwrap().create_channel().await;
		if publish_txns_channel.is_err() {
			error!(target: "consumer","RabbitMQ channel error")
		}
		let publish_token_transfers_channel = conn.as_ref().unwrap().create_channel().await;
		if publish_token_transfers_channel.is_err() {
			error!(target: "consumer","RabbitMQ channel error")
		}

		let queue = consume_channel
			.as_ref()
			.unwrap()
			.queue_declare(
				consume_queue_name,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await;
		if queue.is_err() {
			error!(target: "consumer","RabbitMQ declare queue error")
		}
		let token_transfers_queue = publish_token_transfers_channel
			.as_ref()
			.unwrap()
			.queue_declare(
				normalized_token_transfers_queue_name,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await;
		if token_transfers_queue.is_err() {
			error!(target: "producer","RabbitMQ declare queue error")
		}
		let txns_queue = publish_txns_channel
			.as_ref()
			.unwrap()
			.queue_declare(
				normalized_txns_queue_name,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await;
		if txns_queue.is_err() {
			error!(target: "producer","RabbitMQ declare queue error")
		}

		info!(target: "consumer","RabbitMQ consumer connected successfully");
		Self {
			connection: conn.unwrap(),
			consume_channel: consume_channel.unwrap(),
			publish_txns_channel: publish_txns_channel.unwrap(),
			publish_token_transfers_channel: publish_token_transfers_channel.unwrap(),
			rabbitmq_url: url,
			consume_queue_name: consume_queue_name.to_string(),
			normalized_txns_queue_name: normalized_txns_queue_name.to_string(),
			normalized_token_transfers_queue_name: normalized_token_transfers_queue_name.to_string(),
		}
	}
	pub async fn start(self: Self) {
		let mut consumer = self
			.consume_channel
			.basic_consume(
				&self.consume_queue_name,
				"",
				BasicConsumeOptions {
					no_local: true,
					no_ack: false,
					exclusive: true,
					nowait: false,
				},
				FieldTable::default(),
			)
			.await
			.unwrap();
		while let Some(msg) = consumer.next().await {
			let publish_txns_channel = self.publish_txns_channel.clone();
			let publish_token_transfers_channel = self.publish_token_transfers_channel.clone();
			let normalized_txns_queue_name = self.normalized_txns_queue_name.clone();
			let normalized_token_transfers_queue_name = self.normalized_token_transfers_queue_name.clone();

			tokio::spawn(async move {
				let txs = serde_json::from_str::<Vec<EvmCompleteTxn>>(
					&str::from_utf8(msg.as_ref().unwrap().data.as_slice()).unwrap(),
				)
				.unwrap();
				let normalized_txs: Mutex<Vec<NormalizedTx>> = Mutex::new(vec![]);
				let normalized_token_transfers: Mutex<Vec<NormalizedTokenTransfer>> = Mutex::new(vec![]);

				txs.par_iter().for_each(|elm| {
					let normalized_tx = EvmTxnProcessor::normalize_txn(elm).unwrap();

					normalized_token_transfers
						.lock()
						.unwrap()
						.append(&mut EvmTxnProcessor::get_token_transfers(elm).unwrap());
					normalized_txs.lock().unwrap().push(normalized_tx);
				});

				msg.unwrap().ack(BasicAckOptions::default()).await;

				let normalized_txs_str =
					serde_json::to_string::<Vec<NormalizedTx>>(&normalized_txs.lock().unwrap()).unwrap();
				let normalized_token_transfer_str =
					serde_json::to_string::<Vec<NormalizedTokenTransfer>>(&normalized_token_transfers.lock().unwrap())
						.unwrap();

				info!(
					"Block {:#?} txs were processed",
					usize::from_str_radix(txs[0].txn.block_number.trim_start_matches("0x"), 16).unwrap()
				);
				let published_txn = publish_txns_channel.basic_publish(
					"",
					&normalized_txns_queue_name,
					BasicPublishOptions {
						mandatory: true,
						immediate: false,
					},
					normalized_txs_str.as_bytes(),
					BasicProperties::default(),
				);
				let published_token_transfer = publish_token_transfers_channel.basic_publish(
					"",
					&normalized_token_transfers_queue_name,
					BasicPublishOptions {
						mandatory: true,
						immediate: false,
					},
					normalized_token_transfer_str.as_bytes(),
					BasicProperties::default(),
				);
				join(published_txn, published_token_transfer).await;
				//serde_json::to_string::<Vec<NormalizedTx>>(&normalized_txs);.as_bytes()
				// println!("{:#?}", &str::from_utf8(msg.unwrap().data.as_slice()).unwrap());
			});
		}
	}
}
