// Copyright © 2024 Stephan Kunz

//! The node 'taipei' subscribes to an Image on the topic /columbia and publishes the received value on topic /colorado
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

fn columbia_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let mut value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	value.header.frame_id = value.header.frame_id.replace("Test", "Modified");
	info!("sent: '{}'", &value);
	let _ = ctx.put_with("colorado", value);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps {};
	let agent = Agent::new(properties).config(Config::local()?)?;

	agent.publisher().topic("colorado").add()?;

	agent
		.subscriber()
		.put_callback(columbia_callback)
		.topic("columbia")
		.add()?;

	agent.start().await?;
	Ok(())
}
