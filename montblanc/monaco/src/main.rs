// Copyright © 2024 Stephan Kunz

//! The node 'monaco' subscribes to
//!   - a `Twist` on the topic /congo
//!     and publishes on receive
//!   - a `Float32` on the topic /ohio
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {}

fn congo_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Twist = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::Float32::random();
	info!("sent: '{}'", &msg);
	let msg = Message::encode(&msg);
	let _ = ctx.put("ohio", msg);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("monaco")
		.prefix("workstation")
		.config(&Config::default())?;

	agent.publisher().topic("ohio").add()?;

	agent
		.subscriber()
		.put_callback(congo_callback)
		.selector("robot/congo")
		.add()?;

	agent.start().await?;
	Ok(())
}
