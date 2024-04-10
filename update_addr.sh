#!/usr/bin/env bash

ARG0=`readlink -f $0`
SCRIPT_DIR=`dirname $ARG0`
echo $SCRIPT_DIR
CMD_DIR=$SCRIPT_DIR/cmd
#DEV_IP=192.168.4.10

ADDR_CFG=$1
DEST_IP=$2
echo $DEST_IP

cargo run --bin update_addr --release -- -i ${ADDR_CFG} --addr ${DEST_IP}
sleep 0.$(printf '%04d' $((10000 - 10#$(date +%4N)))); sleep 0.4;
cargo run --bin send_cmd_sync --release -- --addr ${DEST_IP}:2222 --port 2221 --cmd ${CMD_DIR}/ip_addr.yaml

