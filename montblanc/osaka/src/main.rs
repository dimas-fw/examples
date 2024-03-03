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

fn parana_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").parana = Some(value);
	Ok(())
}

fn columbia_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").columbia = Some(value);
	Ok(())
}

fn colorado_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").colorado = Some(value);

	let message = messages::PointCloud2::random();
	ctx.put_with("salween", &message)?;
	info!("sent: '{}'", message);

	let message = messages::LaserScan::random();
	ctx.put_with("godavari", &message)?;
	info!("sent: '{}'", message);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::local(), properties)?;

	agent
		.subscriber()
		.put_callback(parana_callback)
		.msg_type("parana")
		.add()?;

	agent
		.subscriber()
		.put_callback(columbia_callback)
		.msg_type("columbia")
		.add()?;

	agent
		.subscriber()
		.put_callback(colorado_callback)
		.msg_type("colorado")
		.add()?;

	agent.publisher().msg_type("salween").add()?;

	agent.publisher().msg_type("godavari").add()?;

	agent.start().await?;
	Ok(())
}
