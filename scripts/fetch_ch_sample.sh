#!/usr/bin/env bash

IP=$1
ARG0=`realpath $0`
echo $ARG0
SCRIPT_DIR=`dirname $ARG0`

shift 1

echo Backup DbfAmpPhaseCoeff
lftp $IP<<EOF
cd /ata0:0/config
mv DbfAmpPhaseCoeff.dat DbfAmpPhaseCoeff.bak
exit
EOF

echo sending cmd
$SCRIPT_DIR/../target/release/send_cmd_sync --addr "$IP:2222" --port 2221 --cmd $SCRIPT_DIR/../cmd/ch_data.yaml 
echo fetching data
wget ftp://${IP}//ata0:0/config/DbfAmpPhaseCoeff.dat $@

echo restore DbfAmpPhaseCoeff
lftp $IP<<EOF
cd /ata0:0/config
mv DbfAmpPhaseCoeff.bak DbfAmpPhaseCoeff.dat
exit
EOF
