mod processor_pipeline;
mod producer;

use anyhow::Result;
use dotenv::dotenv;
use processor_pipeline::ProcessorPipeline;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();

	let processor_pipeline = ProcessorPipeline::connect(
		std::env::var("AMQP_ADDR").unwrap().to_string(),
		"tx_processor",
		"normalized_txns",
		"normalized_token_transfers",
	)
	.await;
	processor_pipeline.start().await;
	Ok(())
}
