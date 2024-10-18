// Copyright © 2024 Stephan Kunz

//! The node 'lyon' subscribes to a Float32 on the topic /amazon and publishes the received value on topic /tigris
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

async fn amazon_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Float32 = message.decode()?;
	info!("sent: '{}'", &value);
	let value = Message::encode(&value);
	let _ = ctx.put("tigris", value);
	Ok(())
}

#[tokio::main(worker_threads = 3)]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps {};
	let agent = Agent::new(properties)
		.name("lyon")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("tigris").add()?;

	agent
		.subscriber()
		.put_callback(amazon_callback)
		.topic("amazon")
		.add()?;

	agent.start().await?;
	Ok(())
}
