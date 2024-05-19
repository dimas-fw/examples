// Copyright © 2024 Stephan Kunz

//! The node 'freeport' publishes an Int64 value every 50 ms on the topic /ganges
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
		.name("freeport")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("ganges").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(50))
		.callback(|ctx| -> Result<()> {
			let message = messages::Int64::random();
			info!("sent: '{}'", &message);
			let message = Message::encode(&message);
			ctx.put_with("ganges", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
