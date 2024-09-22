#!/usr/bin/env bash

ARG0=`readlink -f $0`
SCRIPT_DIR=`dirname $ARG0`

if [ $# -ne 2 ]
then
    echo Usage: $0 '<HOST> <OP>'
    echo "possible OPs are:"
    echo "coolon -- turn on cooling sys"
    echo "cooloff -- turn off cool sys"
    echo "dbfon -- turn on dbf sys"
    echo "dbfoff -- turn off dbf sys"
    echo "coolstate -- query cooling sys state"
    echo "state -- query all states"
    exit
fi

IP=$1
#msg=$2
OP=$2

echo `date` $IP $OP `w|grep pwr_ctrl`|tee -a ${SCRIPT_DIR}/../pwr_ctrl.log

function send_cmd(){
    _IP=$1
    _msg=$2
    #echo $IP $msg
    echo -n "$msg" |xxd -r -p |nc -v -w 1 $IP 10000 |od -w1024 -t x1 -An 
}

#echo -n "$msg" |xxd -r -p |nc -v -w 1 $IP 10000 |od -t x1

function parse_cool_state(){
    read state
    echo replied: $state
    state=`echo $state|awk '{print $4}'`
    
    case "x$state" in 
        x01)
        echo "on"
        ;;
        x00)
        echo "off"
        ;;
        *)
        echo "unknown"
        echo $state
        ;;
    esac
}


function parse_all_state(){
    read state
    echo replied: $state
    pwr_state=`echo $state|awk '{print $8}'`
    cooling_working_state=`echo $state|awk '{print $12}'`
    pwr_state_cool=`echo $pwr_state/2 |bc`
    pwr_state_dbf=`echo $pwr_state%2 |bc`
    #echo PWR_STATE: $pwr_state_cool $pwr_state_dbf COOLING_WORKING_STATE: $cooling_working_state
    COOL=0
    DBF=0
    [ $pwr_state_cool == 1 ] && COOL=1
    [ $pwr_state_dbf == 1 ] && DBF=1
    echo COOLING: $COOL DBF: $DBF COOLING_WORKING_STATE: $cooling_working_state
}

case "x$OP" in 
    xcoolon)
        echo "Cooling sys on"
        msg="01 05 00 01 FF 00 DD FA"
        send_cmd $IP "msg"
        ;;
    xcooloff)
        echo "Cooling sys off"
        msg="01 05 00 01 00 00 9C 0A"
        send_cmd $IP "msg"
        ;;
    xdbfon)
        echo "Dbf system on"
        msg="01 05 00 00 FF 00 8C 3A"
        send_cmd $IP "msg"
        ;;
    xdbfoff)
        echo "Dbf system off"
        msg="01 05 00 00 00 00 CD CA"
        send_cmd $IP "msg"
        ;;
    xcoolstate)
        echo "Querying cooling system state"
        msg="01 02 00 00 00 05 B8 09"
        send_cmd $IP "msg" |parse_cool_state
        ;;
    xstate)
        echo "Querying all state"
        msg="01 04 03 E8 00 14 70 75"
        send_cmd $IP "msg" |parse_all_state
        ;;
    *)
        echo "unknown OP"
        echo "possible OPs are:"
        echo "coolon -- turn on cooling sys"
        echo "cooloff -- turn off cool sys"
        echo "dbfon -- turn on dbf sys"
        echo "dbfoff -- turn off dbf sys"
        echo "coolstate -- query cooling sys state"
        echo "state -- query all states"
        ;;
esac
