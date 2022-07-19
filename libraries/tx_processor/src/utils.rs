use anyhow::anyhow;
use bigdecimal::{BigDecimal, ToPrimitive};
use ethabi::{
	Contract, Event, EventParam, Hash, LogParam,
	ParamType::{Address, Uint},
};
use num_bigint::BigUint;
use num_traits::Num;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::fmt::Debug;
use std::fs::File;
use std::ops::Div;
use std::str::FromStr;

pub fn hex_str_to_number<T: FromStr>(value: &str) -> T
where
	<T as FromStr>::Err: Debug,
{
	let without_prefix = value.trim_start_matches("0x");
	T::from_str(&BigUint::from_str_radix(without_prefix, 16).unwrap().to_string()).unwrap()
}

pub fn wei_hex_to_usize(value: &str) -> f64 {
	let without_prefix = value.trim_start_matches("0x");
	let normalizer_str = BigDecimal::from_str(&"1000000000000000000").unwrap();
	BigDecimal::from_str(&BigUint::from_str_radix(without_prefix, 16).unwrap().to_string())
		.unwrap()
		.div(normalizer_str)
		.to_f64()
		.unwrap()
}

pub fn normalize_wei(value: usize) -> usize {
	let normalizer_str = BigUint::from_str(&"1000000000000000000").unwrap();
	usize::from_str(&BigUint::from(value).div(normalizer_str).to_string()).unwrap()
}

pub fn decode_token_transfer<T>(topics: &[String], data: &str) -> anyhow::Result<T>
where
	T: DeserializeOwned + std::fmt::Debug,
{
	let event = Event {
		name: "Transfer".to_string(),
		inputs: vec![
			EventParam {
				name: "from".to_string(),
				kind: Address,
				indexed: true,
			},
			EventParam {
				name: "to".to_string(),
				kind: Address,
				indexed: true,
			},
			EventParam {
				name: "value".to_string(),
				kind: Uint(256),
				indexed: false,
			},
		],
		anonymous: false,
	};
	if event.signature() != topics[0].parse().unwrap() {
		return Err(anyhow::Error::msg("Invalid signature"));
	}
	let topics: Vec<Hash> = topics.iter().map(|t| t.parse()).collect::<Result<_, _>>()?;
	let data = hex::decode(data)?;
	let decoded = event.parse_log((topics, data).into())?;

	let mut result = decoded
		.params
		.clone()
		.into_iter()
		.enumerate()
		.map(|(i, log_param)| {
			let log_val = String::from("0x") + &log_param.value.to_string();
			if i != decoded.params.len() - 1 {
				return format!(r#""{}":"{}","#, log_param.name, log_val);
			}
			format!(r#" "{}":"{}" "#, log_param.name, log_val)
		})
		.collect::<Vec<String>>()
		.join("\n");
	result = format!("{} {} {}", "{", result, "}");
	let struct_result = serde_json::from_str::<T>(&result);
	Ok(struct_result.unwrap())
}

pub fn decode_log<T>(path: &str, name_or_signature: &str, topics: &[String], data: &str) -> anyhow::Result<T>
where
	T: DeserializeOwned + std::fmt::Debug,
{
	let event = load_event(path, name_or_signature)?;
	let topics: Vec<Hash> = topics.iter().map(|t| t.parse()).collect::<Result<_, _>>()?;
	let data = hex::decode(data)?;
	let decoded = event.parse_log((topics, data).into())?;

	let mut result = decoded
		.params
		.clone()
		.into_iter()
		.enumerate()
		.map(|(i, log_param)| {
			let log_val = String::from("0x") + &log_param.value.to_string();
			if i != decoded.params.len() - 1 {
				return format!(r#""{}":"{}","#, log_param.name, log_val);
			}
			format!(r#" "{}":"{}" "#, log_param.name, log_val)
		})
		.collect::<Vec<String>>()
		.join("\n");
	result = format!("{} {} {}", "{", result, "}");
	let struct_result = serde_json::from_str::<T>(&result);
	Ok(struct_result.unwrap())
}

fn hash_signature(sig: &str) -> Hash {
	Hash::from_slice(Keccak256::digest(sig.replace(' ', "").as_bytes()).as_slice())
}

fn load_event(path: &str, name_or_signature: &str) -> anyhow::Result<Event> {
	let file = File::open(path)?;
	let contract = Contract::load(file)?;
	let params_start = name_or_signature.find('(');

	match params_start {
		// It's a signature.
		Some(params_start) => {
			let name = &name_or_signature[..params_start];
			let signature = hash_signature(name_or_signature);
			contract
				.events_by_name(name)?
				.iter()
				.find(|event| event.signature() == signature)
				.cloned()
				.ok_or_else(|| anyhow!("Invalid signature `{}`", signature))
		}

		// It's a name.
		None => {
			let events = contract.events_by_name(name_or_signature)?;
			match events.len() {
				0 => unreachable!(),
				1 => Ok(events[0].clone()),
				_ => Err(anyhow!(
					"More than one function found for name `{}`, try providing the full signature",
					name_or_signature
				)),
			}
		}
	}
}

pub fn get_cargo_directory() -> String {
	String::from(env!("CARGO_MANIFEST_DIR"))
}
