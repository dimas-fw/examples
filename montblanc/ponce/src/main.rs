// Copyright © 2024 Stephan Kunz

//! The node 'ponce' subscribes to
//!   - a `StringMsg` on the topic /danube
//!   - a `Pose` on the topic /tagus
//!   - an `Image` on the topic /missouri
//!   - a `PointCloud2` on the topic /brazos
//!   - a `Vector3` on the topic /yamuna
//!   - a `LaserScan` on the topic /godavari
//!   - a `PointCloud2` on the topic /loire
//!   - a `Float32` on the topic /ohio
//!   - a `Float64` on the topic /volga
//! and publishes on receive of tpoic /brazos
//!   - a `Twist` on the topic /congo
//!   - a `TwistWithCovarianceStampe` on the topic /mekong
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct AgentProps {
	danube: Option<messages::StringMsg>,
	tagus: Option<messages::Pose>,
	missouri: Option<messages::Image>,
	godavari: Option<messages::LaserScan>,
	loire: Option<messages::PointCloud2>,
	yamuna: Option<messages::Vector3>,
	ohio: Option<messages::Float32>,
	volga: Option<messages::Float64>,
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

fn missouri_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Image = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").missouri = Some(value);
	Ok(())
}

fn brazos_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::PointCloud2 = message.decode()?;
	info!("received: '{}'", &value);

	let message = messages::Twist::random();
	info!("sent: '{}'", &message);
	ctx.put_with("congo", message)?;

	let message = messages::TwistWithCovarianceStamped::random();
	info!("sent: '{}'", &message);
	ctx.put_with("mekong", message)?;
	Ok(())
}

fn yamuna_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Vector3 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").yamuna = Some(value);
	Ok(())
}

fn godavari_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::LaserScan = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").godavari = Some(value);
	Ok(())
}

fn loire_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::PointCloud2 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").loire = Some(value);
	Ok(())
}

fn ohio_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Float32 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").ohio = Some(value);
	Ok(())
}

fn volga_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::Float64 = message.decode()?;
	info!("received: '{}'", &value);
	ctx.write().expect("should not happen").volga = Some(value);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	init_tracing();

	let properties = AgentProps::default();
	let agent = Agent::new(properties).config(Config::default())?;

	agent.publisher().topic("congo").add()?;

	agent.publisher().topic("mekong").add()?;

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
		.put_callback(missouri_callback)
		.topic("missouri")
		.add()?;

	agent
		.subscriber()
		.put_callback(brazos_callback)
		.topic("brazos")
		.add()?;

	agent
		.subscriber()
		.put_callback(yamuna_callback)
		.topic("yamuna")
		.add()?;

	agent
		.subscriber()
		.put_callback(godavari_callback)
		.topic("godavari")
		.add()?;

	agent
		.subscriber()
		.put_callback(loire_callback)
		.topic("loire")
		.add()?;

	agent
		.subscriber()
		.put_callback(ohio_callback)
		.topic("ohio")
		.add()?;

	agent
		.subscriber()
		.put_callback(volga_callback)
		.topic("volga")
		.add()?;

	agent.start().await?;
	Ok(())
}
