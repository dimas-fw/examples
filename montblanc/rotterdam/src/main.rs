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

fn mekong_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
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
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let agent = Agent::new(properties).config(Config::default())?;

	agent.publisher().topic("murray").add()?;

	agent
		.subscriber()
		.put_callback(mekong_callback)
		.topic("mekong")
		.add()?;

	agent.start().await?;
	Ok(())
}
