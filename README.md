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

## [Montblanc](https://github.com/dimas-fw/examples/tree/main/montblanc)

The montblanc benchmark simulates a robot using several nodes/agents communicating
with a workstation that is running also several nodes/agents.

On Linux you can install `tmux` switch to the `montblanc` directory and then run
```shell
./tmux-robot.sh
```
or
```shell
./tmux-robot.sh --release
```
in one shell and in another shell
```shell
./tmux-workstation.sh
```
or
```shell
./tmux-workstation.sh --release
```
