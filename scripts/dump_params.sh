#!/usr/bin/env bash

for i in $@
do
    for fname in DbfAmpPhaseCoeff DbfAmpPhaseBeamCoeff1 DbfInitCoeff1 AttCtrl Delaydata
    do
        rm -f ${fname}_${i}.dat
        wget -q ftp://${i}_CTRL//ata0:0/config/${fname}.dat -O ${fname}_${i}.dat && echo ${fname}_${i}.dat
    done
done

for i in $@
do
    fname=PortEnable
    rm -f ${fname}_${i}.txt
    wget -q ftp://${i}_CTRL//ata0:0/config/${fname}.txt -O ${fname}_${i}.txt && echo ${fname}_${i}.txt
done
