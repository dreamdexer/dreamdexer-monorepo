use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use log::{error, info};
use std::str;

pub struct LoaderHandler {
	pub connection: Connection,
	pub consume_channel: Channel,
	pub rabbitmq_url: String,
	pub consume_queue_name: String,
}

impl LoaderHandler {
	pub async fn connect(url: &str, consume_queue_name: &str) -> Self {
		let conn = Connection::connect(url, ConnectionProperties::default()).await;
		if conn.is_err() {
			error!(target: "consumer","RabbitMQ connection error")
		}

		let consume_channel = conn.as_ref().unwrap().create_channel().await;
		if consume_channel.is_err() {
			error!(target: "consumer","RabbitMQ channel error")
		}

		let consume_queue = consume_channel
			.as_ref()
			.unwrap()
			.queue_declare(
				consume_queue_name,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await;
		if consume_queue.is_err() {
			error!(target: "consumer","RabbitMQ declare queue error")
		}

		info!(target: "consumer","RabbitMQ consumer connected successfully");
		Self {
			connection: conn.unwrap(),
			consume_channel: consume_channel.unwrap(),
			rabbitmq_url: url.to_string(),
			consume_queue_name: consume_queue_name.to_string(),
		}
	}
}
