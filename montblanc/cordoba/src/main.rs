// Copyright © 2024 Stephan Kunz

//! The node 'cordoba' publishes a Float32 value every 100 ms on the topic /amazon
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps {};
	let mut agent = Agent::new(Config::local()?, properties)?;

	agent.publisher().msg_type("amazon").add()?;

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
