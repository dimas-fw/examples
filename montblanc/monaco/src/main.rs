// Copyright © 2024 Stephan Kunz

//! The node 'monaco' subscribes to
//!   - a `Twist` on the topic /congo
//! and publishes on receive
//!   - a `Float32` on the topic /ohio
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {}

fn congo_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Twist = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::Float32::random();
	info!("sent: '{}'", &msg);
	let _ = ctx.put_with("ohio", msg);
	Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::default(), properties)?;

	agent.publisher().msg_type("ohio").add()?;

	agent
		.subscriber()
		.put_callback(congo_callback)
		.msg_type("congo")
		.add()?;

	agent.start().await?;
	Ok(())
}
