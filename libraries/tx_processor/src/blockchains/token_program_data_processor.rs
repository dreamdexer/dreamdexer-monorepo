use anyhow::Result;
use common::SolanaTxn;
use spl_token_2022::instruction::TokenInstruction;

use crate::TokenTransfer;

use super::instruction_data_processor::InstructionDataProcessor;

pub struct TokenProgramDataProcessor {
	pub address: String,
}

impl Default for TokenProgramDataProcessor {
	fn default() -> Self {
		Self {
			address: String::from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
		}
	}
}
#[allow(deprecated)]
impl InstructionDataProcessor for TokenProgramDataProcessor {
	fn process(
		&self,
		instruction_data: &Vec<u8>,
		txn: &SolanaTxn,
		instruction_accounts: &Vec<usize>,
	) -> Result<Option<TokenTransfer>> {
		let decoded_instruction = TokenInstruction::unpack(instruction_data);
		match decoded_instruction.unwrap() {
			TokenInstruction::MintTo { amount } | TokenInstruction::Burn { amount } => {
				let from = txn.transaction.message.account_keys[instruction_accounts[0]].clone();
				let to = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let mint = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let authority = txn.transaction.message.account_keys[instruction_accounts[2]].clone();
				Ok(Some(TokenTransfer {
					tx_hash: txn.transaction.signatures[0].clone(),
					block_number: txn.slot,
					from,
					to,
					value: amount,
					authority,
					token_name: String::from("unknown"),
					decimals: 0,
					mint_address: mint,
					timestamp: txn.block_time,
				}))
			}
			TokenInstruction::BurnChecked { amount, decimals } | TokenInstruction::MintToChecked { amount, decimals } => {
				let from = txn.transaction.message.account_keys[instruction_accounts[0]].clone();
				let to = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let mint = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let authority = txn.transaction.message.account_keys[instruction_accounts[2]].clone();
				Ok(Some(TokenTransfer {
					tx_hash: txn.transaction.signatures[0].clone(),
					block_number: txn.slot,
					from,
					to,
					value: amount,
					authority,
					token_name: String::from("unknown"),
					decimals,
					mint_address: mint,
					timestamp: txn.block_time,
				}))
			}
			TokenInstruction::TransferChecked { amount, decimals } => {
				let from = txn.transaction.message.account_keys[instruction_accounts[0]].clone();
				let mint = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let to = txn.transaction.message.account_keys[instruction_accounts[2]].clone();
				let authority = txn.transaction.message.account_keys[instruction_accounts[3]].clone();
				Ok(Some(TokenTransfer {
					tx_hash: txn.transaction.signatures[0].clone(),
					block_number: txn.slot,
					from,
					to,
					value: amount,
					authority,
					token_name: String::from("unknown"),
					decimals,
					mint_address: mint,
					timestamp: txn.block_time,
				}))
			}
			TokenInstruction::Transfer { amount } => {
				let from = txn.transaction.message.account_keys[instruction_accounts[0]].clone();
				let to = txn.transaction.message.account_keys[instruction_accounts[1]].clone();
				let authority = txn.transaction.message.account_keys[instruction_accounts[2]].clone();
				Ok(Some(TokenTransfer {
					tx_hash: txn.transaction.signatures[0].clone(),
					block_number: txn.slot,
					from,
					to,
					value: amount,
					authority,
					token_name: String::from("unknown"),
					decimals: 0,
					mint_address: String::from("unknown"),
					timestamp: txn.block_time,
				}))
			}
			_ => Ok(None),
		}
	}
}
