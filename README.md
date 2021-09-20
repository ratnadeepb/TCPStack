# TCP/IP Userspace Stack Implementation

## Environmnet Variables

```bash
export CARGO_TARGET_DIR=./target/
```

## Setting CAP_NET_ADMIN

```bash
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp_stack
```

## Run the Program

```bash
./$CARGO_TARGET_DIR/release/tcp_stack
```
