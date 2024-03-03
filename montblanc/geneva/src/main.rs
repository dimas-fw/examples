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

fn parana_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	let msg = messages::StringMsg {
		data: format!("geneva/arkansas: {}", &value),
	};
	let _ = ctx.put_with("arkansas", &msg);
	info!("sent: '{msg}'");
	Ok(())
}

fn danube_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").danube = Some(value);
	Ok(())
}

fn tagus_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Pose = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").tagus = Some(value);
	Ok(())
}

fn congo_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Twist = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").congo = Some(value);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::default(), properties)?;

	agent.publisher().msg_type("arkansas").add()?;

	agent
		.subscriber()
		.put_callback(parana_callback)
		.msg_type("parana")
		.add()?;

	agent
		.subscriber()
		.put_callback(danube_callback)
		.msg_type("danube")
		.add()?;

	agent
		.subscriber()
		.put_callback(tagus_callback)
		.msg_type("tagus")
		.add()?;

	agent
		.subscriber()
		.put_callback(congo_callback)
		.msg_type("congo")
		.add()?;

	agent.start().await?;
	Ok(())
}
