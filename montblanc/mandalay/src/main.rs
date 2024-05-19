// Copyright © 2024 Stephan Kunz

//! The node 'mandalay' subscribes to
//!   - a `StringMsg` on the topic /danube
//!   - a `Quaternion` on the topic /chenab
//!   - a `PointCloud2` on the topic /salween
//!   - a `LaserScan` on the topic /godavari
//!   - a `Vector3` on the topic /yamuna
//!   - a `PointCloud2` on the topic /loire
//! and publishes every 100ms
//!   - a `Pose` on the topic /tagus
//!   - an `Image` on the topic /missouri
//!   - a `PointCloud2` on the topic /brazos
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::time::Duration;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	danube: Option<messages::StringMsg>,
	chenab: Option<messages::Quaternion>,
	salween: Option<messages::PointCloud2>,
	godavari: Option<messages::LaserScan>,
	loire: Option<messages::PointCloud2>,
	yamuna: Option<messages::Vector3>,
}

fn danube_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").danube = Some(value);
	Ok(())
}

fn chenab_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Quaternion = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").chenab = Some(value);
	Ok(())
}

fn salween_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::PointCloud2 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").salween = Some(value);
	Ok(())
}

fn godavari_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::LaserScan = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").godavari = Some(value);
	Ok(())
}

fn yamuna_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Vector3 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").yamuna = Some(value);
	Ok(())
}

fn loire_callback(ctx: &Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::PointCloud2 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").loire = Some(value);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties)
		.name("mandalay")
		.prefix("robot")
		.config(&Config::default())?;

	agent
		.subscriber()
		.put_callback(danube_callback)
		.topic("danube")
		.add()?;

	agent
		.subscriber()
		.put_callback(chenab_callback)
		.topic("chenab")
		.add()?;

	agent
		.subscriber()
		.put_callback(salween_callback)
		.topic("salween")
		.add()?;

	agent
		.subscriber()
		.put_callback(godavari_callback)
		.topic("godavari")
		.add()?;

	agent
		.subscriber()
		.put_callback(yamuna_callback)
		.topic("yamuna")
		.add()?;

	agent
		.subscriber()
		.put_callback(loire_callback)
		.topic("loire")
		.add()?;

	agent.publisher().topic("tagus").add()?;

	agent.publisher().topic("missouri").add()?;

	agent.publisher().topic("brazos").add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Pose::random();
			info!("sent: '{}'", &message);
			let message = Message::encode(&message);
			ctx.put_with("tagus", message)?;
			Ok(())
		})
		.add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::Image::random();
			let message = Message::encode(&message);
			ctx.put_with("missouri", message)?;
			info!("mandalay sent Image");
			Ok(())
		})
		.add()?;

	agent
		.timer()
		.name("timer")
		.interval(Duration::from_millis(100))
		.callback(|ctx| -> Result<()> {
			let message = messages::PointCloud2::random();
			let message = Message::encode(&message);
			ctx.put_with("brazos", message)?;
			info!("mandalay sent PointCloud2");
			Ok(())
		})
		.add()?;

	agent.start().await?;
	Ok(())
}
