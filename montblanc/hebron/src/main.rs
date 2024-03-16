// Copyright © 2024 Stephan Kunz

//! The node 'hebron' publishes a Quaternion value every 100 ms on the topic /chenab
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

	agent.publisher().msg_type("chenab").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Quaternion::random();
			info!("sent: '{}'", &message);
			ctx.put_with("chenab", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
