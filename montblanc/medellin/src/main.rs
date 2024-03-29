// Copyright © 2024 Stephan Kunz

//! The node 'medellin' publishes an Int32 value every 10 ms on the topic /nilnilee
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
struct AgentProps {}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps {};
	let agent = Agent::new(properties).config(Config::local()?)?;

	agent.publisher().topic("nile").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(50))
		.callback(|ctx| -> Result<()> {
			let message = messages::Int32::random();
			info!("sent: '{}'", &message);
			ctx.put_with("nile", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
