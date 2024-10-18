// Copyright © 2024 Stephan Kunz

//! The node 'medellin' publishes an Int32 value every 10 ms on the topic /nilnilee
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[dimas::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps {};
	let agent = Agent::new(properties)
		.name("medellin")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("nile").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(50))
		.callback(|ctx| -> Result<()> {
			let message = messages::Int32::random();
			info!("sent: '{}'", &message);
			let message = Message::encode(&message);
			ctx.put("nile", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
