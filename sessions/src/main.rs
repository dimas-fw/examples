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
    println!("{} received: name of {} [{}]", name, message, ctx.read()?.0);
    ctx.write()?.0 += 1;
    Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
    init_tracing();

    // create agent 1
    let mut agent1 = Agent::new(AgentProps (0))
        .prefix("agent/1")
        .name("agent1")
        .config(&Config::from_file("tls.json5")?)?;

    agent1.publisher().topic("hostname").add()?;

    agent1
        .subscriber()
        .selector("agent/*/hostname")
        .put_callback(receiver)
        .add()?;

    agent1
        .timer()
        .name("timer")
        .interval(Duration::from_secs(10))
        .callback(sender)
        .add()?;

    agent1.liveliness(true);
    let handle1 = agent1.start();

    // create agent 2
    let mut agent2 = Agent::new(AgentProps(0))
        .prefix("agent/2")
        .name("agent2")
        .config(&Config::from_file("network.json5")?)?;

    agent2.publisher().topic("hostname").add()?;

    agent2
        .timer()
        .name("timer")
        .interval(Duration::from_secs(10))
        .callback(sender)
        .add()?;

    agent2
        .subscriber()
        .selector("agent/*/hostname")
        .put_callback(receiver)
        .add()?;

    agent2.liveliness(true);
    let handle2 = agent2.start();

    // create agent 2
    let mut agent3 = Agent::new(AgentProps(0))
        .prefix("agent/3")
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
        .selector("agent/*/hostname")
        .put_callback(receiver)
        .add()?;

    agent3.liveliness(true);
    let handle3 = agent3.start();

    let _res = tokio::join!(handle1, handle2, handle3);

    Ok(())
}
