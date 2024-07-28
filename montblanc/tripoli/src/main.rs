// Copyright © 2024 Stephan Kunz

//! The node 'tripoli' subscribes to
//! - an `Image` on the topic /columbia
//! - a `LaserScan` on the topic /godavari
//!   and publishes on receive of /godavari a `PointCloud2` on topic /loire
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	columbia: Option<messages::Image>,
}

fn columbia_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	// just to see what has been sent
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").columbia = Some(value);
	Ok(())
}

fn godavari_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::LaserScan = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::PointCloud2::random();
	info!("received: '{}'", &msg);
	let msg = Message::encode(&msg);
	let _ = ctx.put("loire", msg);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("tripoli")
		.prefix("robot")
		.config(&Config::local()?)?;

	agent
		.subscriber()
		.put_callback(columbia_callback)
		.topic("columbia")
		.add()?;

	agent
		.subscriber()
		.put_callback(godavari_callback)
		.topic("godavari")
		.add()?;

	agent.publisher().topic("loire").add()?;

	agent.start().await?;
	Ok(())
}
