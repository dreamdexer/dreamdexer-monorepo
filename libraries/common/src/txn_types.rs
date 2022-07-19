use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct EvmHead {
	pub parent_hash: String,
	pub sha3_uncles: String,
	pub miner: String,
	pub state_root: String,
	pub transactions_root: String,
	pub receipts_root: String,
	pub logs_bloom: String,
	pub difficulty: String,
	pub number: String,
	pub gas_limit: String,
	pub gas_used: String,
	pub timestamp: String,
	pub extra_data: String,
	pub mix_hash: String,
	pub nonce: String,
	pub ext_data_hash: String,
	pub base_fee_per_gas: String,
	pub ext_data_gas_used: String,
	pub block_gas_cost: String,
	pub hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTxnReceiptLogs {
	pub address: String,
	pub topics: Vec<String>,
	pub data: String,
	pub block_number: String,
	pub transaction_hash: String,
	pub transaction_index: String,
	pub block_hash: String,
	pub log_index: String,
	pub removed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTxnReceipt {
	pub block_hash: String,
	pub block_number: String,
	pub contract_address: Option<String>,
	pub cumulative_gas_used: String,
	pub effective_gas_price: String,
	pub from: String,
	pub gas_used: String,
	pub logs: Vec<EvmTxnReceiptLogs>,
	pub logs_bloom: String,
	pub status: String,
	pub to: Option<String>,
	pub transaction_hash: String,
	pub transaction_index: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmCompleteTxn {
	pub txn: EvmTxn,
	pub txn_receipt: EvmTxnReceipt,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct EvmBlock {
	pub base_fee_per_gas: String,
	pub block_extra_data: String,
	pub block_gas_cost: String,
	pub difficulty: String,
	pub ext_data_gas_used: String,
	pub ext_data_hash: String,
	pub extra_data: String,
	pub gas_limit: String,
	pub gas_used: String,
	pub hash: String,
	pub logs_bloom: String,
	pub miner: String,
	pub mix_hash: String,
	pub nonce: String,
	pub number: String,
	pub parent_hash: String,
	pub receipts_root: String,
	pub sha3_uncles: String,
	pub size: String,
	pub state_root: String,
	pub timestamp: String,
	pub total_difficulty: String,
	pub transactions: Vec<EvmTxn>,
	pub transactions_root: String,
	pub uncles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTxn {
	pub block_hash: String,
	pub block_number: String,
	pub from: String,
	pub gas: String,
	pub gas_price: String,
	pub max_fee_per_gas: Option<String>,
	pub max_priority_fee_per_gas: Option<String>,
	pub hash: String,
	pub input: String,
	pub nonce: String,
	pub to: Option<String>,
	pub transaction_index: String,
	pub value: String,
	#[serde(rename = "type")]
	pub tx_type: String,
	pub access_list: Option<Vec<EvmAccessList>>,
	pub chain_id: Option<String>,
	pub timestamp: Option<String>,
	pub v: String,
	pub r: String,
	pub s: String,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmAccessList {
	address: String,
	storage_keys: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxn {
	pub block_time: usize,
	pub meta: Option<SolanaTxnMeta>,
	pub slot: usize,
	pub transaction: SolanaTxnData,
}

/*
	Solana transaction data
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnData {
	pub message: SolanaTxnMessage,
	pub signatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMessage {
	pub account_keys: Vec<String>,
	pub header: SolanaTxnHeader,
	pub instructions: Vec<SolanaTxnInstruction>,
	pub recent_blockhash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnHeader {
	pub num_readonly_signed_accounts: usize,
	pub num_readonly_unsigned_accounts: usize,
	pub num_required_signatures: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnInstruction {
	pub accounts: Vec<usize>,
	pub data: String,
	pub program_id_index: usize,
}

/*
Solana txn meta
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMeta {
	pub err: Option<SolanaTxError>,
	pub fee: usize,
	pub log_messages: Option<Vec<String>>,
	pub inner_instructions: Option<Vec<SolanaTxnMetaInnerInstruction>>,
	pub post_balances: Vec<usize>,
	pub post_token_balances: Option<Vec<SolanaTxnMetaTokenBalanceStatus>>,
	pub pre_balances: Vec<usize>,
	pub pre_token_balances: Option<Vec<SolanaTxnMetaTokenBalanceStatus>>,
	pub status: Option<SolanaTxnMetaStatus>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolanaTxError {
	AccountInUse,
	AccountLoadedTwice,
	AccountNotFound,
	ProgramAccountNotFound,
	InsufficientFundsForFee,
	InvalidAccountForFee,
	AlreadyProcessed,
	BlockhashNotFound,
	InstructionError(u8, SolanaInstructionError),
	CallChainTooDeep,
	MissingSignatureForFee,
	InvalidAccountIndex,
	SignatureFailure,
	InvalidProgramForExecution,
	SanitizeFailure,
	ClusterMaintenance,
	AccountBorrowOutstanding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolanaInstructionError {
	GenericError,
	InvalidArgument,
	InvalidInstructionData,
	InvalidAccountData,
	AccountDataTooSmall,
	InsufficientFunds,
	IncorrectProgramId,
	MissingRequiredSignature,
	AccountAlreadyInitialized,
	UninitializedAccount,
	UnbalancedInstruction,
	ModifiedProgramId,
	ExternalAccountLamportSpend,
	ExternalAccountDataModified,
	ReadonlyLamportChange,
	ReadonlyDataModified,
	DuplicateAccountIndex,
	ExecutableModified,
	RentEpochModified,
	NotEnoughAccountKeys,
	AccountDataSizeChanged,
	AccountNotExecutable,
	AccountBorrowFailed,
	AccountBorrowOutstanding,
	DuplicateAccountOutOfSync,
	Custom(u32),
	InvalidError,
	ExecutableDataModified,
	ExecutableLamportChange,
	ExecutableAccountNotRentExempt,
	UnsupportedProgramId,
	CallDepth,
	MissingAccount,
	ReentrancyNotAllowed,
	MaxSeedLengthExceeded,
	InvalidSeeds,
	InvalidRealloc,
	ComputationalBudgetExceeded,
	PrivilegeEscalation,
	ProgramEnvironmentSetupFailure,
	ProgramFailedToComplete,
	ProgramFailedToCompile,
	Immutable,
	IncorrectAuthority,
	BorshIoError(String),
	AccountNotRentExempt,
	InvalidAccountOwner,
	ArithmeticOverflow,
	UnsupportedSysvar,
	IllegalOwner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMetaStatus {
	pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaUiTokenAmount {
	pub amount: String,
	pub decimals: u8,
	pub ui_amount: Option<f32>,
	pub ui_amount_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMetaTokenBalanceStatus {
	pub account_index: u8,
	pub mint: String,
	pub owner: Option<String>,
	pub ui_token_amount: SolanaUiTokenAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMetaLoadedAddresses {
	pub readonly: Vec<String>,
	pub writable: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxnMetaInnerInstruction {
	pub index: usize,
	pub instructions: Vec<SolanaTxnInstruction>,
}
