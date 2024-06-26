#!/bin/bash
# Copyright © 2024 Stephan Kunz
# can be called with option `--release` to run release builds

export ZENOH_RUNTIME='(app: (worker_threads: 2),tx: (max_blocking_threads: 1))'

session="workstation"

tmux new-session -d -s $session

window=0
tmux rename-window -t $session:$window 'workstation'

# split window into 16 panes according to following schema
#
#  +-------------+-------------+-------------+
#  |             |             |             |
#  |      0      |      3      |      6      |
#  |             |             |             |
#  +-------------+-------------+-------------+
#  |             |             |             |
#  |      1      |      4      |      7      |
#  |             |             |             |
#  +-------------+-------------+-------------+
#  |             |             |             |
#  |      2      |      5      |      8      |
#  |             |             |             |
#  +-------------+-------------+-------------+

tmux split-pane -h -p 70
tmux split-pane -h -p 50

tmux select-pane -t 0
tmux split-pane -v -p 70
tmux split-pane -v -p 50

tmux select-pane -t 3
tmux split-pane -v -p 70
tmux split-pane -v -p 50

tmux select-pane -t 6
tmux split-pane -v -p 70
tmux split-pane -v -p 50

# start a job in each pane but panes 0, 4, 8
# end with active pane 4
tmux select-pane -t 1
tmux send-keys "cargo run --bin geneva $1" C-m
tmux select-pane -t 2
tmux send-keys "cargo run --bin monaco $1" C-m
tmux select-pane -t 3
tmux send-keys "cargo run --bin rotterdam $1" C-m
tmux select-pane -t 4
tmux send-keys "tail -F /tmp/montblanc.out" C-m
tmux select-pane -t 5
tmux send-keys "cargo run --bin arequipa $1" C-m
tmux select-pane -t 6
tmux send-keys "cargo run --bin barcelona $1" C-m
tmux select-pane -t 7
tmux send-keys "cargo run --bin georgetown $1" C-m

tmux select-pane -t 0

tmux attach-session -t $session
