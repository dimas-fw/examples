// Copyright © 2024 Stephan Kunz

//! The node 'cordoba' publishes a Float32 value every 100 ms on the topic /amazon
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps {};
	let agent = Agent::new(properties)
		.name("cordoba")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("amazon").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Float32::random();
			info!("sent: '{}'", &message);
			ctx.put_with("amazon", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
