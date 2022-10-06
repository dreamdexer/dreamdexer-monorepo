use super::instruction_parser::InstructionParser;
use super::token_program_data_processor::TokenProgramDataProcessor;
use crate::{NormalizedTx, TokenTransfer};

use anyhow::Result;
use common::{BlockchainName, SolanaTxn};
use rayon::prelude::*;
use std::sync::Mutex;

#[derive(Debug)]
pub struct SolanaTxnProcessor {
	pub blockchain_name: BlockchainName,
}

impl SolanaTxnProcessor {
	// type TxnType = SolanaTxn;
	fn get_token_transfers(&self, txn: &SolanaTxn) -> Result<Option<Vec<TokenTransfer>>> {
		let token_program_processor = TokenProgramDataProcessor::default();
		InstructionParser.parse(txn, token_program_processor.address.clone(), &token_program_processor)
	}

	fn normalize_txn(&self, txn: &SolanaTxn) -> Result<NormalizedTx> {
		return Ok(NormalizedTx {
			value: 0,
			block_number: txn.slot.clone(),
			timestamp: txn.block_time.clone(),
			tx_hash: txn.transaction.signatures[0].clone(),
			from: txn.transaction.message.account_keys[0].clone(),
			to: txn
				.transaction
				.message
				.instructions
				.clone()
				.into_iter()
				.map(|elm| txn.transaction.message.account_keys[elm.program_id_index].clone())
				.collect(),
			fee: txn.meta.clone().unwrap().fee.clone(),
			error: false, //  match txn.meta.clone().unwrap().err {
			              //     None=> false,
			              //     _=> true
			              // }
		});
	}

	fn process_txns(&self, txns: &Vec<SolanaTxn>) -> Result<()> {
		let normalized_txs: Mutex<Vec<NormalizedTx>> = Mutex::new(vec![]);
		let token_transfers_par: Mutex<Vec<TokenTransfer>> = Mutex::new(vec![]);

		txns.par_iter().for_each(|txn| {
			let normalized_tx = self.normalize_txn(txn).unwrap();
			let token_transfers_result = self.get_token_transfers(&txn);

			if !token_transfers_result.as_ref().unwrap().is_none() {
				token_transfers_par
					.lock()
					.unwrap()
					.append(&mut token_transfers_result.unwrap().unwrap());
			}
			normalized_txs.lock().unwrap().push(normalized_tx);

			// let extract_features_tx = self.normalize_txn(txn).unwrap();
			// extract_features_txs.lock().unwrap().push(extract_features_tx);
		});

		Ok(())
	}
}

impl SolanaTxnProcessor {}
