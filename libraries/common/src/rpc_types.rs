use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcReqBody {
	pub jsonrpc: String,
	pub id: u8,
	pub method: String,
	pub params: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct RpcResponseBody<T> {
	pub jsonrpc: String,
	pub id: u8,
	pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponseMessage<T> {
	pub jsonrpc: String,
	pub method: String,
	pub params: WebSocketResponseMessageParams<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponseMessageParams<T> {
	pub subscription: String,
	pub result: T,
}
