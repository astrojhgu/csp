#![feature(atomic_bool_fetch_not)]
use clap::Parser;

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

use csp::{
    cfg::{NCH_PER_STREAM, PKT_LEN},
    cspch::{calc_coeff, Correlator, CspChannelizer},
    data_frame::{CorrDataQueue, DbfDataFrame},
    utils::write_data,
};

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    hash::RandomState,
    io::Write,
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    sync::atomic::{AtomicBool, Ordering},
};

use socket2::{Domain, Socket, Type};

use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct Cfg {
    pub src_ip: Vec<String>,
    pub out_prefix: String,
    pub n_fine_ch_eff: usize,
    pub tap: usize,
    pub k: f32,
    pub cnt: usize,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('c'), long("cfg"), value_name = "cfg file")]
    cfg_file: String,
}

const NPKT_PER_CORR: usize = (1 << 18);

fn main() {
    let args = Args::parse();
    let cfg: Cfg = from_reader(std::fs::File::open(&args.cfg_file).unwrap()).unwrap();

    let src_addrs: HashMap<SocketAddrV4, usize, RandomState> = cfg
        .src_ip
        .iter()
        .map(|s| s.parse::<SocketAddrV4>().unwrap())
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();
    let n_stations = src_addrs.len();

    let mut dump_files: Vec<_> = (0..n_stations)
        .map(|i| {
            let fname = format!("dump_{}.dat", i);
            File::create(&fname).unwrap()
        })
        .collect();

    println!("{:?}", src_addrs);
    //std::process::exit(0);

    println!("{:?}", cfg.src_ip);

    
    let addr: std::net::SocketAddr = "0.0.0.0:4001".parse().unwrap();
    let udp_socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    udp_socket.bind(&addr.into()).unwrap();
    udp_socket.set_recv_buffer_size(100 * 1024 * 1024).unwrap();
    udp_socket.set_nonblocking(true).unwrap();
    println!("{}", udp_socket.recv_buffer_size().unwrap());
    let udp_socket: UdpSocket = udp_socket.into();

    //let udp_socket = UdpSocket::bind("0.0.0.0:4001").unwrap();
    //let mut old_pkt_id = 0;
    //let mut pkt_cnt = 0;
    let mut data = DbfDataFrame::default();
    let udp_buf = unsafe {
        std::slice::from_raw_parts_mut((&mut data) as *mut DbfDataFrame as *mut u8, 8080)
    };

    let mut buf = vec![
        vec![0_i16; NCH_PER_STREAM * 2 * csp::cfg::NFRAME_PER_PKT * NPKT_PER_CORR];
        n_stations
    ];

    let mut pkt_id_buf=vec![
        vec![0_usize; NPKT_PER_CORR];
        n_stations
    ];

    let mut corr_id_list = vec![0; n_stations];
    let mut old_pkt_id = vec![None; n_stations];

    loop {
        let (_, src_addr) = loop {
            match udp_socket.recv_from(udp_buf) {
                Ok((len, src_addr)) if len == PKT_LEN => break (len, src_addr),
                Err(_) => {}
                _ => {}
            }
        };



        //println!("src_addr:{}", src_addr);
        match src_addr {
            SocketAddr::V4(s) => match src_addrs.get(&s) {
                Some(&i) => {
                    let pkt_id=data.pkt_id as usize;
                    if let Some(old_pkt_id1)=old_pkt_id[i]{
                        if old_pkt_id1+1!=pkt_id{
                            println!("{} pkts dropped from {} {} {}", pkt_id-old_pkt_id1-1, s, old_pkt_id1, pkt_id);
                        }
                    }

                    old_pkt_id[i]=Some(pkt_id);
                    

                    let corr_id = data.pkt_id as usize / NPKT_PER_CORR;
                    //println!("{}", corr_id);
                    let next_corr_id = (data.pkt_id + 1) as usize / NPKT_PER_CORR;
                    let offset = (data.pkt_id as usize - corr_id * NPKT_PER_CORR)
                        * NCH_PER_STREAM
                        * 2
                        * csp::cfg::NFRAME_PER_PKT;

                    buf[i][offset..offset + NCH_PER_STREAM * 2 * csp::cfg::NFRAME_PER_PKT]
                        .copy_from_slice(&data.payload);

                    pkt_id_buf[i][data.pkt_id as usize - corr_id * NPKT_PER_CORR]=data.pkt_id as usize;

                    if next_corr_id == corr_id + 1 && next_corr_id <= 1{
                        corr_id_list[i] = corr_id;
                        let mut outfile =
                            File::create(format!("/dev/shm/d_{}_{}.dat", i, corr_id)).unwrap();
                        write_data(&mut outfile, &buf[i]);
                        let mut outfile=File::create(format!("/dev/shm/pkt_id_{}_{}.dat", i, corr_id)).unwrap();
                        write_data(&mut outfile, &pkt_id_buf[i]);

                        buf[i].fill(0);
                        pkt_id_buf[i].fill(0);

                        if corr_id_list.iter().max() == corr_id_list.iter().min()
                            && next_corr_id == 10
                        {
                            break;
                        }
                    }
                }
                None => {
                    panic!("unregistered station addr");
                }
            },
            SocketAddr::V6(_s) => continue,
        }
    }
}
