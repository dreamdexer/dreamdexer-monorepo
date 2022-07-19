use anyhow::Result;
use common::SolanaTxn;

use crate::TokenTransfer;

pub trait InstructionDataProcessor {
	fn process(
		self: &Self,
		instruction_data: &Vec<u8>,
		account_keys: &SolanaTxn,
		instruction_accounts: &Vec<usize>,
	) -> Result<Option<TokenTransfer>>;
}
