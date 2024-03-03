// Copyright © 2024 Stephan Kunz

//! The node 'delhi' publishes an Image value every 1 s on the topic /columbia
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

	agent.publisher().msg_type("columbia").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_secs(1))
		.callback(|ctx| -> Result<(), DimasError> {
			let message = messages::Image::random();
			ctx.put_with("columbia", &message)?;
			// just to see what has been sent
			info!("sent: '{message}'");
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
