// Copyright © 2024 Stephan Kunz

//! The node 'hamburg' subscribes to
//!   - a Float32 on the topic /tigris
//!   - an Int64 on the topic /ganges
//!   - an Int32 on the topic /nile
//!   - a String on the topic /danube
//! and publishes the on /danube received value on topic /parana
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	ganges: i64,
	nile: i32,
	tigris: f32,
}

fn tigris_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Float32 = message.decode()?;
	ctx.write().expect("should not happen").tigris = value.data;
	info!("received: '{}'", &value);
	Ok(())
}

fn ganges_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Int64 = message.decode()?;
	ctx.write().expect("should not happen").ganges = value.data;
	info!("received: '{}'", &value);
	Ok(())
}

fn nile_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::Int32 = message.decode()?;
	ctx.write().expect("should not happen").nile = value.data;
	info!("received: '{}'", &value);
	Ok(())
}

fn danube_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<(), DimasError> {
	let value: messages::StringMsg = message.decode()?;
	let msg = messages::StringMsg {
		data: format!("hamburg/parana: {}", &value.data),
	};
	let _ = ctx.put_with("parana", &msg);
	info!("sent: '{msg}'");
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DimasError> {
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::default(), properties)?;

	agent.publisher().msg_type("parana").add()?;

	agent
		.subscriber()
		.put_callback(tigris_callback)
		.msg_type("tigris")
		.add()?;

	agent
		.subscriber()
		.put_callback(ganges_callback)
		.msg_type("ganges")
		.add()?;

	agent
		.subscriber()
		.put_callback(nile_callback)
		.msg_type("nile")
		.add()?;

	agent
		.subscriber()
		.put_callback(danube_callback)
		.msg_type("danube")
		.add()?;

	agent.start().await?;
	Ok(())
}
