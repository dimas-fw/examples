// Copyright © 2024 Stephan Kunz

//! The node 'georgetown' subscribes to
//!   - a `WrenchStamped` on the topic /lena
//!   - a `Vector3Stamped` on the topic /murray
//!     and publishes every 50ms
//!   - a `Float64` on the topic /volga
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	murray: Option<messages::Vector3Stamped>,
	lena: Option<messages::WrenchStamped>,
	volga: Option<messages::Float64>,
}

async fn lena_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::WrenchStamped = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().lena = Some(value);
	Ok(())
}

async fn murray_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Vector3Stamped = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().murray = Some(value);
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("georgetown")
		.prefix("workstation")
		.config(&Config::default())?;

	agent.publisher().topic("volga").add()?;

	agent
		.subscriber()
		.put_callback(lena_callback)
		.topic("lena")
		.add()?;

	agent
		.subscriber()
		.put_callback(murray_callback)
		.topic("murray")
		.add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(50))
		.callback(|ctx| -> Result<()> {
			let message = messages::Float64::random();
			let value = message.data;
			let msg = Message::encode(&message);
			ctx.write().volga = Some(message);
			ctx.put("volga", msg)?;
			info!("sent: '{value}'");
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
