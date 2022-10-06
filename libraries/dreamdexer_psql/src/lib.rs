use anyhow::Result;
use common::{NormalizedTokenTransfer, NormalizedTx};
// use common::NormalizedTx;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
use std::sync::Arc;
use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Row, Socket};

pub async fn establish_connection(
	host: &str,
	db_name: &str,
	user: &str,
	password: &str,
) -> (Client, Connection<Socket, NoTlsStream>) {
	tokio_postgres::connect(
		&format!("host={host} user={user} password={password} dbname={db_name}"),
		NoTls,
	)
	.await
	.expect("Could't connect to DB")
}

pub async fn create_txn(db_client: Arc<Client>, txn: &NormalizedTx) -> Result<Vec<Row>> {
	let response = db_client
		.query(
			&format!(
				"INSERT INTO txns (hash, block_number, value, from_address, to_address,error, fee)
			VALUES ('{}', {},'{}', '{}', {}, '{}', '{}');",
				txn.hash,
				txn.block_number,
				txn.value,
				txn.from,
				if txn.to.is_none() {
					("null").to_string()
				} else {
					format!("'{}'", txn.to.as_ref().unwrap()).to_string()
				},
				txn.error,
				txn.fee
			),
			&[],
		)
		.await;
	if response.is_err() {
		Err(anyhow::format_err!("Error inserting txn {}: {:#?}", txn.hash, response))
	} else {
		Ok(response.unwrap())
	}
}
pub async fn create_token_transfer(
	db_client: Arc<Client>,
	token_transfer: &NormalizedTokenTransfer,
) -> Result<Vec<Row>> {
	let response = db_client
		.query(
			&format!(
				"INSERT INTO token_transfers (tx_hash, block_number, value, from_address, to_address,token_name, token_ticker, token_address, decimals)
			VALUES ('{}', {},'{}', '{}', '{}', '{}','{}', '{}',{});",
				token_transfer.tx_hash,
				token_transfer.block_number,
				token_transfer.value,
				token_transfer.from,
				token_transfer.to,
				token_transfer.token_name,
				token_transfer.token_ticker,
				token_transfer.token_address,
				token_transfer.decimals
			),
			&[],
		)
		.await;
	if response.is_err() {
		Err(anyhow::format_err!(
			"Error inserting token_transfer {}: {:#?}",
			token_transfer.tx_hash,
			response
		))
	} else {
		Ok(response.unwrap())
	}
}
