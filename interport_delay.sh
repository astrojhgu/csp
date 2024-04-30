#!/usr/bin/env bash
source utils.sh

CTRL_IP=192.168.4.10
PAYLOAD_IP1=192.168.1.32
PAYLOAD_IP2=192.168.1.42

mkdir -p interport_delay

function test_one_port {
    port=$1
    if [ $port -lt 0 ] || [ $port -gt 127 ]
    then
       echo "port should >=0 && <=127"
       exit 
    fi
    
    others_att=$3
    out_prefix=$4
    enable_port tmp_wgt_cfg1.yaml 127 
    enable_port tmp_wgt_cfg2.yaml $port 

    ./update_wgt.sh tmp_wgt_cfg1.yaml tmp_wgt_cfg2.yaml $CTRL_IP

    write_corr_cfg tmp_corr_cfg.yaml ${PAYLOAD_IP1} ${PAYLOAD_IP2} $out_prefix

    cargo run --bin send_cmd_sync --release -- --addr 192.168.4.255:2222 --port 2221 --cmd cmd/stop.yaml
    cargo run --bin send_cmd_sync --release -- --addr 192.168.4.255:2222 --port 2221 --cmd cmd/sync.yaml

    cargo run --bin corr_fine_udp --release -- --cfg ./tmp_corr_cfg.yaml
}

if [ $# -ge 1 ]
then
    :
else
    echo "Port missing"
    exit
fi

port1=$1
if [ $# -gt 1 ]
then 
    port2=$2
else
    port2=$port1
fi

echo $port1 $port2 

all_off_att_cfg tmp_att.yaml
./update_att.sh tmp_att.yaml $CTRL_IP

for port in `seq ${port1} ${port2}`
do
    rm -rf ./interport_delay/on_${port}*
    rm -rf ./interport_delay/off_${port}*

    all_off_att_cfg tmp_att.yaml
    ./update_att.sh tmp_att.yaml $CTRL_IP
    test_one_port $port 0 30 ./interport_delay/off_${port} || exit

    all_pass_att_cfg tmp_att.yaml
    ./update_att.sh tmp_att.yaml $CTRL_IP
    test_one_port $port 0 30 ./interport_delay/on_${port} || exit

    ./plot_interport_delay.py ${port} || exit
done
