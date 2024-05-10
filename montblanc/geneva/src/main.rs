// Copyright © 2024 Stephan Kunz

//! The node 'geneva' subscribes to
//!   - a `StringMsg` on the topic /parana
//!   - a `StringMsg` on the topic /danube
//!   - a `Pose` on the topic /tagus
//!   - a `Twist` on the topic /congo
//! and publishes on receive of topic /parana
//!   - a `StringMsg` on the topic /arkansas
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	danube: Option<messages::StringMsg>,
	tagus: Option<messages::Pose>,
	congo: Option<messages::Twist>,
}

fn parana_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::StringMsg {
		data: format!("geneva/arkansas: {}", &value),
	};
	info!("sent: '{}'", &msg);
	let _ = ctx.put_with("arkansas", msg);
	Ok(())
}

fn danube_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").danube = Some(value);
	Ok(())
}

fn tagus_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Pose = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").tagus = Some(value);
	Ok(())
}

fn congo_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Twist = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").congo = Some(value);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties).name("geneva").config(&Config::default())?;

	agent.publisher().topic("arkansas").add()?;

	agent
		.subscriber()
		.put_callback(parana_callback)
		.topic("parana")
		.add()?;

	agent
		.subscriber()
		.put_callback(danube_callback)
		.topic("danube")
		.add()?;

	agent
		.subscriber()
		.put_callback(tagus_callback)
		.topic("tagus")
		.add()?;

	agent
		.subscriber()
		.put_callback(congo_callback)
		.topic("congo")
		.add()?;

	agent.start().await?;
	Ok(())
}
