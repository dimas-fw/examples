// Copyright © 2024 Stephan Kunz

//! The node 'barcelona' subscribes to
//!   - a `TwistWithCovarianceStamed` on the topic /mekong
//! and publishes on receive the Twist data as
//!   - a `WrenchStamped` on the topic /lena
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {}

fn mekong_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::TwistWithCovarianceStamped = message.decode()?;
	info!("received: '{}'", &value);
	let wrench = messages::Wrench {
		force: value.twist.twist.linear,
		torque: value.twist.twist.angular,
	};
	let msg = messages::WrenchStamped {
		header: value.header,
		wrench,
	};
	info!("sent: '{}'", &msg);
	let _ = ctx.put_with("lena", msg);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties).name("barcelona").config(&Config::default())?;

	agent.publisher().topic("mekong").add()?;

	agent
		.subscriber()
		.put_callback(mekong_callback)
		.topic("mekong")
		.add()?;

	agent.start().await?;
	Ok(())
}
