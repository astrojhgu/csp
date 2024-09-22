#!/usr/bin/env bash

IP=$1
ARG0=`realpath $0`
echo $ARG0
SCRIPT_DIR=`dirname $ARG0`

shift 1

$SCRIPT_DIR/../target/release/send_cmd_sync --addr "$IP:2222" --port 2221 --cmd $SCRIPT_DIR/../cmd/t_sample.yaml && rm -f ADdata.dat

wget ftp://${IP}//ata0:0/config/ADdata.dat $@

