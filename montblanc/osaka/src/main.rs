// Copyright © 2024 Stephan Kunz

//! The node 'osaka' subscribes to
//!   - a `String` on the topic /parana
//!   - an `Image` on the topic /columbia
//!   - an `Image` on the topic /colorado
//! and publishes on  receive of /colorado
//!   - a `PointCloud2` on the topic /salween
//!   - a `LaserScan` on the topic /godavari
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	parana: Option<messages::StringMsg>,
	columbia: Option<messages::Image>,
	colorado: Option<messages::Image>,
}

fn parana_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").parana = Some(value);
	Ok(())
}

fn columbia_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").columbia = Some(value);
	Ok(())
}

fn colorado_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").colorado = Some(value);

	let message = messages::PointCloud2::random();
	info!("sent: '{}'", &message);
	ctx.put_with("salween", message)?;

	let message = messages::LaserScan::random();
	info!("sent: '{}'", &message);
	ctx.put_with("godavari", message)?;
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties).config(Config::local()?)?;

	agent
		.subscriber()
		.put_callback(parana_callback)
		.topic("parana")
		.add()?;

	agent
		.subscriber()
		.put_callback(columbia_callback)
		.topic("columbia")
		.add()?;

	agent
		.subscriber()
		.put_callback(colorado_callback)
		.topic("colorado")
		.add()?;

	agent.publisher().topic("salween").add()?;

	agent.publisher().topic("godavari").add()?;

	agent.start().await?;
	Ok(())
}
