use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties, ExchangeKind};
use log::error;
pub struct Producer {
	pub connection: Connection,
	pub channel: Channel,
	pub rabbitmq_url: String,
	pub exchange_name: String,
}

impl Producer {
	pub async fn connect(url: String) -> Self {
		let conn = Connection::connect(&url, ConnectionProperties::default()).await;
		if conn.is_err() {
			error!(target: "producer","RabbitMQ connection error")
		}
		let channel = conn.as_ref().unwrap().create_channel().await;
		if channel.is_err() {
			error!(target: "producer","RabbitMQ channel error")
		}
		let exchange_name = String::from("evm_collector");
		let exchange = channel
			.as_ref()
			.unwrap()
			.exchange_declare(
				&exchange_name,
				ExchangeKind::Fanout,
				ExchangeDeclareOptions {
					passive: false,
					durable: true,
					auto_delete: false,
					internal: false,
					nowait: false,
				},
				FieldTable::default(),
			)
			.await;
		if exchange.is_err() {
			error!(target: "producer","RabbitMQ exchange error")
		}

		let queue = channel
			.as_ref()
			.unwrap()
			.queue_declare("tx_processor", QueueDeclareOptions::default(), FieldTable::default())
			.await;
		if queue.is_err() {
			error!(target: "producer","RabbitMQ declare queue error")
		}
		let bind = channel
			.as_ref()
			.unwrap()
			.queue_bind(
				"tx_processor",
				&exchange_name,
				"",
				QueueBindOptions { nowait: true },
				FieldTable::default(),
			)
			.await;
		if bind.is_err() {
			error!(target: "producer","RabbitMQ bind queue error")
		}
		Self {
			connection: conn.unwrap(),
			channel: channel.unwrap(),
			rabbitmq_url: url,
			exchange_name,
		}
	}
}
