#!/usr/bin/env bash

ARG0=`readlink -f $0`
SCRIPT_DIR=`dirname $ARG0`
echo $SCRIPT_DIR
CMD_DIR=$SCRIPT_DIR/cmd
#DEV_IP=192.168.4.10

WGT_CFG=$1
shift 1
DEST_IPS=$@
echo $DEST_IPS

cargo run --bin calc_wgt --release -- --cfg $WGT_CFG -o /dev/shm/wgt.bin
BC_IP="192.168.4.255"

for IP in $DEST_IPS
do
    cargo run --bin update_wgt --release -- -b 2 -a ${IP} -i /dev/shm/wgt.bin
done

sleep 0.$(printf '%04d' $((10000 - 10#$(date +%4N)))); sleep 0.4;
cargo run --bin send_cmd_sync --release -- --addr ${BC_IP}:2222 --port 2221 --cmd ${CMD_DIR}/03_wgt.yaml

