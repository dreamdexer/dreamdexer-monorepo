mod processor_pipeline;
mod producer;

use anyhow::Result;
use common::NormalizedTx;
use dotenv::dotenv;
use processor_pipeline::ProcessorPipeline;
use producer::Producer;
use std::sync::mpsc::channel;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();

	let consumer = ProcessorPipeline::connect(
		std::env::var("AMQP_ADDR").unwrap().to_string(),
		"tx_processor",
		"normalized_txns",
		"normalized_token_transfers",
	)
	.await;
	consumer.start().await;
	Ok(())
}
