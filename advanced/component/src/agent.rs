//! `DiMAS` components example
//! Copyright © 2024 Stephan Kunz

// region:		--- modules
use dimas::prelude::*;
// endregion:	--- modules

#[derive(Debug)]
struct AgentProps {}

#[dimas::main]
async fn main() -> Result<()> {
    // initialize tracing/logging
    init_tracing();

    // create & initialize agents properties
    let properties = AgentProps {};

    // create an agent with the properties and the prefix 'examples'
    let mut agent = Agent::new(properties)
        .prefix("examples")
        .name("agent")
        .config(&Config::default())?;

    agent.load_library("components")?;

    // activate liveliness
    agent.liveliness(true);

    // run agent
    agent = agent.start().await?;

    agent.unload_library("components")?;

    Ok(())
}
