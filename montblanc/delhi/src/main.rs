// Copyright © 2024 Stephan Kunz

//! The node 'delhi' publishes an Image value every 1 s on the topic /columbia
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
		.name("dehli")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("columbia").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_secs(1))
		.callback(|ctx| -> Result<()> {
			let message = messages::Image::random();
			info!("sent: '{}'", &message);
			ctx.put_with("columbia", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
