// Copyright © 2024 Stephan Kunz

//! The node 'arequipa' subscribes to a `StringMsg` on the topic /arkansas and writes the data to a file
//!
//! This source is part of `DiMAS` implementation of Montblanc benchmark for distributed systems

use dimas::prelude::*;
use std::fs::File;
use std::io::Write;
use tracing::{error, info};

#[cfg(target_os = "windows")]
static OUT_FILE: &str = "c:/tmp/montblanc.out";

#[cfg(not(target_os = "windows"))]
static OUT_FILE: &str = "/tmp/montblanc.out";

#[derive(Debug)]
struct AgentProps {
	file: File,
}

async fn arkansas_callback(ctx: Context<AgentProps>, message: Message) -> Result<()> {
	let value: messages::StringMsg = message.decode()?;
	info!("received: '{}'", &value.data);
	let final_data = format!("{}\n", value.data);
	ctx.write()
		.file
		.write_all(final_data.as_bytes())
		.expect("should not happen");
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	init_tracing();

	let file = File::create(OUT_FILE).unwrap_or_else(|_| {
		error!("Could not create {OUT_FILE}");
		panic!("Could not create {OUT_FILE}");
	});
	let properties = AgentProps { file };
	let agent = Agent::new(properties)
		.name("arequipa")
		.prefix("workstation")
		.config(&Config::local()?)?;

	agent
		.subscriber()
		.put_callback(arkansas_callback)
		.topic("arkansas")
		.add()?;

	agent.start().await?;
	Ok(())
}
