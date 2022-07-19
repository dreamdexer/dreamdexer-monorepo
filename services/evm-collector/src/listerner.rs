use std::sync::mpsc::Sender;

use blockchain_api::EvmApi;

pub struct Listener<'a> {
	pub api: &'a EvmApi,
	pub sender: &'a Sender<usize>,
}

impl<'a> Listener<'a> {
	pub fn new(api: &'a EvmApi, sender: &'a Sender<usize>) -> Self {
		Self { api, sender }
	}

	pub fn listen_new_block(self: &Self) {
		self.api.suscribe_new_head(self.sender);
	}
}
