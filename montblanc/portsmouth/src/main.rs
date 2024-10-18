// Copyright © 2024 Stephan Kunz

//! The node 'portsmouth' publishes a String value every 200 ms on the topic /danube
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
		.name("portsmouth")
		.prefix("robot")
		.config(&Config::default())?;

	agent.publisher().topic("danube").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(200))
		.callback(move |ctx| -> Result<()> {
			let value = "portsmouth/danube: ".to_string() + &messages::random_string(55);
			let message = messages::StringMsg { data: value };
			info!("sent: '{}'", &message);
			let message = Message::encode(&message);
			ctx.put("danube", message)?;
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
