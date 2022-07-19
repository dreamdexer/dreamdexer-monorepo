#[macro_use]
extern crate dreamdexer_psql;

mod loader_handler;
use anyhow::Result;
use common::{NormalizedTokenTransfer, NormalizedTx};
use dotenv::dotenv;
use dreamdexer_psql::{create_token_transfer, create_txn};
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable};
use loader_handler::LoaderHandler;
use log::{error, info};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();

	// Start listening messages

	// process messages async (Insert on db)

	let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
	let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
	let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
	let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

	let rabbitmq_url = env::var("RABBIT_URL").expect("RABBIT_URL must be set");
	let consume_txs_queue_name = env::var("CONSUME_TXS_QUEUE_NAME").expect("CONSUME_TXS_QUEUE_NAME must be set");
	let consume_token_transfers_queue_name =
		env::var("CONSUME_TOKEN_TRANSFERS_QUEUE_NAME").expect("CONSUME_TOKEN_TRANSFERS_QUEUE_NAME must be set");
	// Connect to db
	let (client, connection) =
		dreamdexer_psql::establish_connection(&db_host, &db_name, &db_username, &db_password).await;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}
	});
	let mut threads = vec![];
	let txs_handler = LoaderHandler::connect(&rabbitmq_url, &consume_txs_queue_name).await;

	let arc_client = Arc::new(client);
	let arc_client2 = arc_client.clone();

	let txns_thread = tokio::spawn(async move {
		let mut consumer = txs_handler
			.consume_channel
			.basic_consume(
				&txs_handler.consume_queue_name,
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
			let txs = serde_json::from_str::<Vec<NormalizedTx>>(
				&std::str::from_utf8(msg.as_ref().unwrap().data.as_slice()).unwrap(),
			)
			.unwrap();
			for tx in txs {
				let db_client_clone = arc_client.clone();
				tokio::spawn(async move {
					create_txn(db_client_clone, &tx).await.expect("ERROR INSERTING TX");
				});
			}
			msg.unwrap().ack(BasicAckOptions::default()).await;
		}
	});

	let token_transfers_handler = LoaderHandler::connect(&rabbitmq_url, &consume_token_transfers_queue_name).await;

	let token_transfers_thread = tokio::spawn(async move {
		let mut consumer = token_transfers_handler
			.consume_channel
			.basic_consume(
				&token_transfers_handler.consume_queue_name,
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
			let txs = serde_json::from_str::<Vec<NormalizedTokenTransfer>>(
				&std::str::from_utf8(msg.as_ref().unwrap().data.as_slice()).unwrap(),
			)
			.unwrap();
			for tx in txs {
				let db_client_clone = arc_client2.clone();
				tokio::spawn(async move {
					create_token_transfer(db_client_clone, &tx)
						.await
						.expect("ERROR INSERTING TOKEN TRANSFER");
				});
			}
			msg.unwrap().ack(BasicAckOptions::default()).await;
		}
	});

	threads.push(txns_thread);
	threads.push(token_transfers_thread);
	for t in threads.into_iter() {
		t.await;
	}

	// let mut handlers = vec![];
	// let mut_client = Arc::new(client);

	// println!("DB CONNECTED ");
	// for i in 0..100000 {
	// 	let client_clone = mut_client.clone();

	// 	let new_handler = tokio::spawn(async move {
	//
	// 		println!("{}", i)
	// 	});
	// 	handlers.push(new_handler);
	// }

	// for h in handlers.into_iter() {
	// 	h.await.unwrap();
	// }
	Ok(())
}
