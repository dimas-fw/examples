// Copyright © 2024 Stephan Kunz

//! The node 'kingston' publishes a Vector3 value every 100 ms on the topic /yamuna
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps {};
	let mut agent = Agent::new(Config::local(), properties)?;

	agent.publisher().msg_type("yamuna").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<(), DimasError> {
			let message = messages::Vector3::random();
			ctx.put_with("yamuna", &message)?;
			// just to see what value has been sent
			info!("sent: '{message}'");
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
