#!/usr/bin/env bash
source utils.sh

CTRL_IP=192.168.4.12

mkdir -p port_matching

function test_one_port {
    port=$1
    if [ $port -lt 0 ] || [ $port -gt 127 ]
    then
       echo "port should >=0 && <=127"
       exit 
    fi
    self_att=$2
    others_att=$3
    out_prefix=$4
    enable_port tmp_wgt_cfg.yaml $port 
    ./update_wgt.sh tmp_wgt_cfg.yaml tmp_wgt_cfg.yaml $CTRL_IP
    
    write_att_cfg $port $self_att $others_att tmp_att.yaml
    ./update_att.sh tmp_att.yaml $CTRL_IP

    write_corr_cfg tmp_corr_cfg.yaml 192.168.1.72 192.168.1.82 $out_prefix

    cargo run --bin send_cmd_sync --release -- --addr 192.168.4.255:2222 --port 2221 --cmd cmd/stop.yaml

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
for port in `seq ${port1} ${port2}`
do

    rm -rf ./port_matching/on_${port}*
    rm -rf ./port_matching/off_${port}*
    test_one_port $port 0 30 ./port_matching/on_${port} || exit
    test_one_port $port 30 0 ./port_matching/off_${port} || exit

    ./plot_port_matching.py ${port} || exit

done
