// Copyright © 2024 Stephan Kunz

//! The node 'osaka' subscribes to
//!   - a `String` on the topic /parana
//!   - an `Image` on the topic /columbia
//!   - an `Image` on the topic /colorado
//!     and publishes on  receive of /colorado
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

async fn parana_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().parana = Some(value);
	Ok(())
}

async fn columbia_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().columbia = Some(value);
	Ok(())
}

async fn colorado_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().colorado = Some(value);

	let message = messages::PointCloud2::random();
	info!("sent: '{}'", &message);
	let message = Message::encode(&message);
	ctx.put("salween", message)?;

	let message = messages::LaserScan::random();
	info!("sent: '{}'", &message);
	let message = Message::encode(&message);
	ctx.put("godavari", message)?;
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("osaka")
		.prefix("robot")
		.config(&Config::local()?)?;

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
