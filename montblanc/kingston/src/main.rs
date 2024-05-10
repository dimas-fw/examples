// Copyright © 2024 Stephan Kunz

//! The node 'kingston' publishes a Vector3 value every 100 ms on the topic /yamuna
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
	let agent = Agent::new(properties).name("kingston").config(&Config::local()?)?;

	agent.publisher().topic("yamuna").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Vector3::random();
			info!("sent: '{}'", &message);
			ctx.put_with("yamuna", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
