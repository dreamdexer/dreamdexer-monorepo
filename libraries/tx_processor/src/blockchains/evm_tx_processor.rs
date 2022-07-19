// use super::instruction_parser::InstructionParser;
// use super::token_program_data_processor::TokenProgramDataProcessor;
use crate::{
	blockchains::get_avalanche_tokens_map,
	types::TokenTransferParams,
	utils::{decode_token_transfer, hex_str_to_number, wei_hex_to_usize},
	TxnProcessor,
};
use anyhow::Result;
use common::{EvmCompleteTxn, NormalizedTx, TokenTransfer};

use rayon::prelude::*;
use serde_json::{from_value, json};
use std::sync::Mutex;
// "Transfer(address,address,uint256)";
use common::NormalizedTokenTransfer;

#[derive(Debug)]
pub struct EvmTxnProcessor {}

impl EvmTxnProcessor {}
impl TxnProcessor for EvmTxnProcessor {
	type TxnType = EvmCompleteTxn;

	fn get_token_transfers(txn: &Self::TxnType) -> Result<Vec<NormalizedTokenTransfer>> {
		// let mut token_transfers_dir = get_cargo_directory();
		// token_transfers_dir.push_str("/src/transfer_abi.json");
		let tokens_map = get_avalanche_tokens_map();

		let mut t_transfer: Vec<NormalizedTokenTransfer> = vec![];
		for log in txn.txn_receipt.logs.iter() {
			let log_topics = log
				.topics
				.iter()
				.map(|elm| elm.trim_start_matches("0x").to_string())
				.collect::<Vec<String>>();
			let decoded_log =
				decode_token_transfer::<TokenTransferParams>(&log_topics, &log.data.trim_start_matches("0x"));
			if decoded_log.is_err() {
				continue;
			}

			let token_data = &tokens_map[log.address.clone()];
			let mut mapped_token_ticker = String::from("UNKNOWN");
			let mut mapped_decimals = 18;
			let mut mapped_token_name = String::from("UNKNOWN");

			if !token_data.is_null() {
				mapped_token_ticker = from_value::<String>(token_data["ticker"].clone()).unwrap();
				mapped_decimals = from_value::<u8>(token_data["decimals"].clone()).unwrap();
				mapped_token_name = from_value::<String>(token_data["name"].clone()).unwrap();
			}
			t_transfer.push(NormalizedTokenTransfer {
				tx_hash: txn.txn.hash.clone(),
				block_number: hex_str_to_number::<i32>(&txn.txn_receipt.block_number),
				value: hex_str_to_number::<String>(&decoded_log.as_ref().unwrap().value),
				from: decoded_log.as_ref().unwrap().from.clone(),
				to: decoded_log.as_ref().unwrap().to.clone(),
				token_ticker: mapped_token_ticker,
				token_name: mapped_token_name,
				token_address: log.address.to_string(),
				decimals: mapped_decimals,
			})
		}
		Ok(t_transfer)
	}
	//TODO: HANDLE TX ERROR FROM COLLECTOR
	fn normalize_txn(txn: &Self::TxnType) -> Result<NormalizedTx> {
		return Ok(NormalizedTx {
			value: hex_str_to_number::<String>(&txn.txn.value.clone()),
			block_number: hex_str_to_number::<i32>(&txn.txn_receipt.block_number),
			hash: txn.txn_receipt.transaction_hash.clone(),
			from: txn.txn_receipt.from.clone(),
			to: txn.txn_receipt.to.clone(),
			fee: (wei_hex_to_usize(&txn.txn_receipt.gas_used) * hex_str_to_number::<f64>(&txn.txn.gas_price))
				.to_string(),
			error: match u8::from_str_radix(&txn.txn_receipt.status.trim_start_matches("0x"), 16).unwrap() {
				1 => false,
				_ => true,
			},
		});
	}

	fn process_txns(&self, txns: &Vec<Self::TxnType>) -> Result<()> {
		let normalized_txs: Mutex<Vec<NormalizedTx>> = Mutex::new(vec![]);
		let token_transfers_par: Mutex<Vec<TokenTransfer>> = Mutex::new(vec![]);

		txns.par_iter().for_each(|txn| {
			// let normalized_tx = self.normalize_txn(txn).unwrap();
			// let token_transfers_result = self.get_token_transfers(&txn);

			// if !token_transfers_result.as_ref().unwrap().is_none() {
			// 	token_transfers_par
			// 		.lock()
			// 		.unwrap()
			// 		.append(&mut token_transfers_result.unwrap().unwrap());
			// }
			// normalized_txs.lock().unwrap().push(normalized_tx);

			// let extract_features_tx = self.normalize_txn(txn).unwrap();
			// extract_features_txs.lock().unwrap().push(extract_features_tx);
		});

		Ok(())
	}
}

impl EvmTxnProcessor {}
