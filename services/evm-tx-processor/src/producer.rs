use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use log::{error, info};
pub struct Producer {
	pub connection: Connection,
	pub channel: Channel,
	pub rabbitmq_url: String,
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

		let queue = channel
			.as_ref()
			.unwrap()
			.queue_declare(
				"evm_tx_processor_publish",
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await;
		if queue.is_err() {
			error!(target: "producer","RabbitMQ declare queue error")
		}
		info!(target:"producer", "RabbitMQ producer connected successfully");
		Self {
			connection: conn.unwrap(),
			channel: channel.unwrap(),
			rabbitmq_url: url,
		}
	}
}
