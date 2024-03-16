// Copyright © 2024 Stephan Kunz

//! The node 'arequipa' subscribes to a `StringMsg` on the topic /arkansas and writes the data to a file
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::fs::File;
use std::io::Write;
use tracing::{error, info};

static OUT_FILE: &str = "/tmp/montblanc.out";

#[derive(Debug)]
struct AgentProps {
	file: File,
}

fn arkansas_callback(ctx: &ArcContext<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value.data);
	let final_data = format!("{}\n", value.data);
	ctx.write()
		.expect("should not happen")
		.file
		.write_all(final_data.as_bytes())
		.expect("should not happen");
	Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let file = File::create(OUT_FILE).unwrap_or_else(|_| {
		error!("Could not create {OUT_FILE}");
		panic!("Could not create {OUT_FILE}");
	});
	let properties = AgentProps { file };
	let mut agent = Agent::new(Config::local()?, properties)?;

	agent
		.subscriber()
		.put_callback(arkansas_callback)
		.msg_type("arkansas")
		.add()?;

	agent.start().await?;
	Ok(())
}
