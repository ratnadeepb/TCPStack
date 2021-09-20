#!/bin/bash
cargo b --release
export CARGO_TARGET_DIR=./target/
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp_stack
./$CARGO_TARGET_DIR/release/tcp_stack &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" SIGINT SIGTERM
wait $pid