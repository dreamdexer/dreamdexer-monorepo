use super::instruction_data_processor::InstructionDataProcessor;
use crate::TokenTransfer;
use anyhow::Result;
use bs58::decode;
use common::{SolanaTxn, SolanaTxnInstruction};

pub struct InstructionParser;

impl InstructionParser {
	pub fn parse(
		self: &Self,
		txn: &SolanaTxn,
		program_address: String,
		instruction_data_processor: &impl InstructionDataProcessor,
	) -> Result<Option<Vec<TokenTransfer>>> {
		let program_index = txn
			.transaction
			.message
			.account_keys
			.iter()
			.position(|r| r.to_string() == program_address);
		if program_index.is_none() {
			return Ok(None);
		}
		let mut parsed_instructions: Vec<TokenTransfer> = vec![];

		for (i, instruction) in txn.transaction.message.instructions.iter().enumerate() {
			let processed_instruction =
				self.process_instruction(txn, &instruction, program_index.unwrap(), instruction_data_processor);
			if !processed_instruction.as_ref().unwrap().is_none() {
				let processed_instruction_clone = processed_instruction.unwrap().unwrap().clone();
				parsed_instructions.push(processed_instruction_clone);
			}

			if !txn.meta.is_none()
				&& !txn.meta.as_ref().unwrap().inner_instructions.is_none()
				&& !txn
					.meta
					.as_ref()
					.unwrap()
					.inner_instructions
					.as_ref()
					.unwrap()
					.is_empty()
			{
				for inner_instruction in txn.meta.as_ref().unwrap().inner_instructions.as_ref().unwrap() {
					if i == inner_instruction.index {
						for instruction in inner_instruction.instructions.iter() {
							let processed_instruction =
								self.process_instruction(txn, &instruction, program_index.unwrap(), instruction_data_processor);
							if !processed_instruction.as_ref().unwrap().is_none() {
								parsed_instructions.push(processed_instruction.unwrap().unwrap());
							}
						}
						break;
					}
				}
			}
		}
		Ok(Some(parsed_instructions))
	}

	fn process_instruction(
		self: &Self,
		txn: &SolanaTxn,
		instruction: &SolanaTxnInstruction,
		program_index: usize,
		instruction_data_processor: &impl InstructionDataProcessor,
	) -> Result<Option<TokenTransfer>> {
		let data = decode(instruction.data.clone()).into_vec()?;
		let program_id_index = instruction.program_id_index;
		let mut processed_instruction: Option<TokenTransfer> = None;
		if program_id_index == program_index {
			processed_instruction = instruction_data_processor
				.process(&data, txn, &instruction.accounts)
				.unwrap();
		}
		Ok(processed_instruction)
	}
}
