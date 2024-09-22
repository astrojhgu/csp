#!/usr/bin/env bash

IP=$1
ARG0=`realpath $0`
echo $ARG0
SCRIPT_DIR=`dirname $ARG0`

#shift 1
wget ftp://${IP}//ata0:0/config/DbfAmpPhaseCoeff.dat -O DbfAmpPhaseCoeff.dat

${SCRIPT_DIR}/fit_delay.py DbfAmpPhaseCoeff.dat 0
