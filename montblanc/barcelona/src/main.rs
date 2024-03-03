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

fn mekong_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
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
	let _ = ctx.put_with("lena", &msg);
	info!("sent: '{msg}'");
	Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::default(), properties)?;

	agent.publisher().msg_type("mekong").add()?;

	agent
		.subscriber()
		.put_callback(mekong_callback)
		.msg_type("mekong")
		.add()?;

	agent.start().await?;
	Ok(())
}
