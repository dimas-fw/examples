// Copyright © 2024 Stephan Kunz

//! The node 'freeport' publishes an Int64 value every 50 ms on the topic /ganges
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

	agent.publisher().msg_type("ganges").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(50))
		.callback(|ctx| -> Result<(), DimasError> {
			let message = messages::Int64::random();
			ctx.put_with("ganges", &message)?;
			// just to see what value has been sent
			info!("sent: '{message}'");
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
