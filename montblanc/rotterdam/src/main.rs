// Copyright © 2024 Stephan Kunz

//! The node 'rotterdam' subscribes to
//!   - a `TwistWithCovarianceStamped` on the topic /mekong
//! and publishes on receive
//!   - a `Vector3Stamped` on the topic /murray
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {}

fn mekong_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::TwistWithCovarianceStamped = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::Vector3Stamped {
		header: value.header,
		vector: value.twist.twist.linear,
	};
	info!("sent: '{}'", &msg);
	let _ = ctx.put_with("murray", msg);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("rotterdam")
		.prefix("workstation")
		.config(&Config::default())?;

	agent.publisher().topic("murray").add()?;

	agent
		.subscriber()
		.put_callback(mekong_callback)
		.key_expr("robot/mekong")
		.add()?;

	agent.start().await?;
	Ok(())
}
