// Copyright © 2024 Stephan Kunz

//! The node 'lyon' subscribes to a Float32 on the topic /amazon and publishes the received value on topic /tigris
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

fn amazon_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Float32 = message.decode()?;
	let _ = ctx.put_with("tigris", &value);
	info!("sent: '{value}'");
	Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps {};
	let mut agent = Agent::new(Config::local(), properties)?;

	agent.publisher().msg_type("tigris").add()?;

	agent
		.subscriber()
		.put_callback(amazon_callback)
		.msg_type("amazon")
		.add()?;

	agent.start().await?;
	Ok(())
}
