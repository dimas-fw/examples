//! `DiMAS` router example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

/// The Router's properties
#[derive(Debug)]
pub struct RouterProps {}

#[dimas::main]
async fn main() -> Result<()> {
    // initialize tracing/logging
    init_tracing();

    // create & initialize  router properties
    let properties = RouterProps {};

    // create an agent with the properties and router configuration
    let mut router = Agent::new(properties)
        .prefix("examples")
        .name("router")
        .config(&Config::router()?)?;

    // activate liveliness
    router.liveliness(true);
    // start the router agent
    router.start().await?;
    Ok(())
}
