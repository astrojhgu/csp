function enable_port {
outfile=$1
shift
ports=$@
cat <<EOF >$outfile
delay: [
  0,0,0,0,0,0,0,0 # 0-7
  ,0,0,0,0,0,0,0,0 #8-15
  ,0,0,0,0,0,0,0,0 #16-23
  ,0,0,0,0,0,0,0,0 #24-31
  ,0,0,0,0,0,0,0,0 #32-39
  ,0,0,0,0,0,0,0,0 #40-47
  ,0,0,0,0,0,0,0,0 #48-55
  ,0,0,0,0,0,0,0,0 #56-63
  ,0,0,0,0,0,0,0,0 #64-71
  ,0,0,0,0,0,0,0,0 #72-79
  ,0,0,0,0,0,0,0,0 #80-87
  ,0,0,0,0,0,0,0,0 #88-95
  ,0,0,0,0,0,0,0,0 #96-103
  ,0,0,0,0,0,0,0,0 #104-111
  ,0,0,0,0,0,0,0,0 #112-119
  ,0,0,0,0,0,0,0,0 #120-127
]
ampl: [
  1,1,1,1,1,1,1,1 # 0-7
  ,1,1,1,1,1,1,1,1 #8-15
  ,1,1,1,1,1,1,1,1 #16-23
  ,1,1,1,1,1,1,1,1 #24-31
  ,1,1,1,1,1,1,1,1 #32-39
  ,1,1,1,1,1,1,1,1 #40-47
  ,1,1,1,1,1,1,1,1 #48-55
  ,1,1,1,1,1,1,1,1 #56-63
  ,1,1,1,1,1,1,1,1 #64-71
  ,1,1,1,1,1,1,1,1 #72-79
  ,1,1,1,1,1,1,1,1 #80-87
  ,1,1,1,1,1,1,1,1 #88-95
  ,1,1,1,1,1,1,1,1 #96-103
  ,1,1,1,1,1,1,1,1 #104-111
  ,1,1,1,1,1,1,1,1 #112-119
  ,1,1,1,1,1,1,1,1 #120-127
]
flags: !Enable [ $@ ]
EOF
}

function write_att_cfg {
    port=$1
    self_att=$2
    others_att=$3
    outfile=$4
    echo value: >$outfile
    for i in `seq 0 127`
    do
        if [ $i -eq $port ]
        then
            echo "    - $self_att" >>$outfile
        else
            echo "    - $others_att" >>$outfile
        fi
    done
}

function all_pass_att_cfg {
    outfile=$1
    echo value: >$outfile
    for i in `seq 0 127`
    do
        
        echo "    - 0" >>$outfile
    done
}

function all_off_att_cfg {
    outfile=$1
    echo value: >$outfile
    for i in `seq 0 127`
    do
        
        echo "    - 31" >>$outfile
    done
}


function write_corr_cfg {
    outfile=$1
    ip1=$2
    ip2=$3
    outprefix=$4
    cat <<EOF >$outfile
src_addr:
  - ${ip1}:4002
  - ${ip2}:4002
dst_addr: 0.0.0.0:4002
out_prefix: ${outprefix}
n_fine_ch_eff: 16
tap: 32
k: 0.9
cnt: 10
EOF
}
