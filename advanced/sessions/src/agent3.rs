//! `DiMAS` sessions example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

/// Counter to track number of receives
#[derive(Debug)]
struct AgentProps(u128);

fn sender(ctx: Context<AgentProps>) -> Result<()> {
    let hostname = hostname::get()?
        .into_string()
        .unwrap_or_else(|_| String::from("unknown host"));
    #[allow(clippy::unwrap_used)]
    let name = ctx.name().unwrap();
    let msg = format!("{name} is {hostname}");
    let message = Message::encode(&msg);
    let _ = ctx.put("hostname", message);

    Ok(())
}

async fn receiver(ctx: Context<AgentProps>, message: Message) -> Result<()> {
    #[allow(clippy::unwrap_used)]
    let name = ctx.name().unwrap();
    let message: String = message.decode()?;
    println!("{} received: hostname of {} [{}]", name, message, ctx.read()?.0);
    ctx.write()?.0 += 1;
    Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
    init_tracing();

    let mut agent3 = Agent::new(AgentProps(0))
        .prefix("agent/tcp")
        .name("agent3")
        .config(&Config::from_file("local.json5")?)?;

    agent3.publisher().topic("hostname").add()?;

    agent3
        .timer()
        .name("timer")
        .interval(Duration::from_secs(10))
        .callback(sender)
        .add()?;

    agent3
        .subscriber()
        .topic("hostname")
        .put_callback(receiver)
        .add()?;

    agent3.liveliness(true);
    agent3.start().await?;

    Ok(())
}
