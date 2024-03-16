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
	tracing_subscriber::fmt::init();

	let properties = AgentProps::default();
	let mut agent = Agent::new(Config::default(), properties)?;

	agent.publisher().msg_type("congo").add()?;

	agent.publisher().msg_type("mekong").add()?;

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
		.put_callback(missouri_callback)
		.msg_type("missouri")
		.add()?;

	agent
		.subscriber()
		.put_callback(brazos_callback)
		.msg_type("brazos")
		.add()?;

	agent
		.subscriber()
		.put_callback(yamuna_callback)
		.msg_type("yamuna")
		.add()?;

	agent
		.subscriber()
		.put_callback(godavari_callback)
		.msg_type("godavari")
		.add()?;

	agent
		.subscriber()
		.put_callback(loire_callback)
		.msg_type("loire")
		.add()?;

	agent
		.subscriber()
		.put_callback(ohio_callback)
		.msg_type("ohio")
		.add()?;

	agent
		.subscriber()
		.put_callback(volga_callback)
		.msg_type("volga")
		.add()?;

	agent.start().await?;
	Ok(())
}
