// Copyright © 2024 Stephan Kunz

//! The node 'hebron' publishes a Quaternion value every 100 ms on the topic /chenab
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[tokio::main(worker_threads = 3)]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps {};
	let agent = Agent::new(properties)
		.name("hebron")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent.publisher().topic("chenab").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Quaternion::random();
			info!("sent: '{}'", &message);
			let message = Message::encode(&message);
			ctx.put("chenab", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
