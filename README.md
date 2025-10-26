# DiMAS Examples

Examples using DiMAS, the **Di**stributed **M**ulti **A**gent **S**ystem framework.

## [Ping/Pong](https://github.com/dimas-fw/examples/tree/main/pingpong)

Implements a Ping/Pong pair that measures the roundtrip time. The Ping does not
wait for a started Pong, but continues pinging.

Run the ping'er in one terminal window with

```shell
cargo run --bin ping
```

and the pong'er in another terminal window with

```shell
cargo run --bin pong
```

## [Queryable/Querier](https://github.com/dimas-fw/examples/tree/main/queries)

Implements a Queryable/Querier pair, where the Querier does not wait for
a started Queryable, but continues querying. On the querier side there are 
used different kinds of query callbacks. 

Run the querier in one terminal window with

```shell
cargo run --bin querier
```

and the queryable in another terminal window with

```shell
cargo run --bin queryable
```

## [Sessions](https://github.com/dimas-fw/examples/tree/main/sessions)

Implements 4 agents using different communication configurations. 
This example shows the difficulties to separate visibility within a clique.

Run each agent in a different terminal window with

```shell
cargo run --bin agentX
```
where X is 1, 2, 3 or 4


## [Montblanc](https://github.com/dimas-fw/examples/tree/main/montblanc)

The montblanc benchmark simulates a robot using several nodes/agents communicating
with a workstation that is running also several nodes/agents.
