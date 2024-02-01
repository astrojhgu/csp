#![feature(atomic_bool_fetch_not)]
use clap::Parser;

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

use csp::{
    cfg::{NCH_PER_STREAM, NFRAME_PER_CORR, PKT_LEN},
    cspch::{calc_coeff, Correlator, CspChannelizer},
    data_frame::{CorrDataQueue, DbfDataFrame},
    utils::write_data,
};

use std::{
    collections::HashMap, fs::OpenOptions, hash::RandomState, io::Write, net::{SocketAddr, SocketAddrV4, UdpSocket}, sync::atomic::{AtomicBool, Ordering}
};

use socket2::{Socket, Domain, Type};

use chrono::prelude::*;


#[derive(Serialize, Deserialize)]
struct Cfg{
    pub src_ip:Vec<String>
    , pub out_prefix: String
    , pub n_fine_ch_eff: usize
    , pub tap: usize
    , pub k: f32
    , pub cnt: usize
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(
        short('c'),
        long("cfg"),
        value_name = "cfg file",
    )]
    cfg_file: String,
    
}

fn main() {
    let args = Args::parse();
    let cfg:Cfg=from_reader(std::fs::File::open(&args.cfg_file).unwrap()).unwrap();

    
    let src_addrs:HashMap<SocketAddrV4, usize, RandomState> = cfg.src_ip
        .iter()
        .map(|s| s.parse::<SocketAddrV4>().unwrap())
        .enumerate().map(|(a,b)|(b,a)).collect();
    
    
    println!("{:?}", src_addrs);
    //std::process::exit(0);

    println!("{:?}", cfg.src_ip);

    let nfine_eff = cfg.n_fine_ch_eff;
    let nfine_full = nfine_eff * 2;

    let coeffs = calc_coeff(nfine_full, cfg.tap, cfg.k);
    let mut channelizers = src_addrs
        .iter()
        .map(|_| CspChannelizer::new(NFRAME_PER_CORR, NCH_PER_STREAM, nfine_full, &coeffs))
        .collect::<Vec<_>>();

    let out_prefix = cfg.out_prefix;

    

    let n_stations = src_addrs.len();
    let (mut corr_queue, receiver): (Vec<_>, Vec<_>) =
        (0..n_stations).map(|_| CorrDataQueue::new()).unzip();

    let running=std::sync::Arc::new(AtomicBool::new(true));
    let running1=std::sync::Arc::clone(&running);
    std::thread::spawn(move || {
        let addr:std::net::SocketAddr="0.0.0.0:4001".parse().unwrap();
        let udp_socket=Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        udp_socket.bind(&addr.into()).unwrap();
        udp_socket.set_recv_buffer_size(100*1024*1024).unwrap();
        //udp_socket.set_nonblocking(true).unwrap();
        println!("{}", udp_socket.recv_buffer_size().unwrap());
        let udp_socket:UdpSocket=udp_socket.into();

        //let udp_socket = UdpSocket::bind("0.0.0.0:4001").unwrap();
        //let mut old_pkt_id = 0;
        //let mut pkt_cnt = 0;
        let mut data = DbfDataFrame::default();
        let buf = unsafe {
            std::slice::from_raw_parts_mut((&mut data) as *mut DbfDataFrame as *mut u8, 8080)
        };
        loop {
            let (_, src_addr) = 
            loop{
                match udp_socket.recv_from(buf){
                    Ok((len, src_addr)) if len==PKT_LEN=>break (len, src_addr),
                    Err(_)=>{
                        
                    }
                    _=>{}
                }
            };

            
            if !running1.load(Ordering::SeqCst){
                break;
            }
            
            //println!("src_addr:{}", src_addr);
            match src_addr {
                SocketAddr::V4(s) => match src_addrs.get(&s) {
                    Some(i) => {
                        corr_queue[*i].push(&data)},
                    None => {
                        panic!("unregistered station addr");
                    }
                },
                SocketAddr::V6(_s) => {
                    continue},
            }
        }
    });

    //let mut channelized_data = vec![0_f32; channelizers[0].output_buf_len()];
    let mut correlator = Correlator::new(NCH_PER_STREAM * nfine_eff, NFRAME_PER_CORR / nfine_full);
    let mut corr_data = vec![0f32; NCH_PER_STREAM * nfine_eff * 2];
    let mut idx = 0;
    loop {
        let mut corr_id_list = Vec::new();
        let mut max_corr_id = 0;
        receiver
            .iter()
            .zip(channelizers.iter_mut()).enumerate()
            .for_each(|(i, (r, channelizer1))| {
                let x = r.recv().unwrap();
                println!("{} {}", x.corr_id, receiver.len());
                corr_id_list.push(x.corr_id);
                max_corr_id = max_corr_id.max(x.corr_id);
                //channelizer1.channelize(&x.payload, &mut channelized_data);
                /*
                let fname=format!("dump_{}.dat", i);
                let mut dump = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .write(true)
                        .open(&fname)
                        .unwrap();
                write_data(&mut dump, &x.payload);
                */
                channelizer1.channelize_no_out(&x.payload);
                //let fname = format!("{}_{}.dat", args.out_prefix, dn);
                //let mut outfile = std::fs::File::create(fname).unwrap();
                //write_data(&mut outfile, &channelized_data);
            });

        corr_id_list
            .iter()
            .zip(
                receiver
                    .iter()
                    .zip(channelizers.iter_mut()),
            )
            .for_each(|(&cid, (r, channelizer1))| {
                if cid != max_corr_id {
                    println!("{} mismatch", cid);
                    let x = r.recv().unwrap();
                    println!("{} {}", x.corr_id, receiver.len());
                    //channelizer1.channelize(&x.payload, &mut channelized_data);
                    channelizer1.channelize_no_out(&x.payload);
                    //let fname = format!("{}_{}.dat", args.out_prefix, dn);
                    //let mut outfile = std::fs::File::create(fname).unwrap();
                    //write_data(&mut outfile, &channelized_data);
                }
            });

        let now = Utc::now();
        let time_i64 = now.timestamp();
        let time_filename = format!("{out_prefix}_time.txt");
        let mut time_file = OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(&time_filename)
            .unwrap();

        writeln!(&mut time_file, "{}", time_i64).unwrap();

        let time_filename = format!("{out_prefix}_time.bin");
        let mut time_file = OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(&time_filename)
            .unwrap();

        let b = time_i64.to_le_bytes();
        time_file.write_all(&b).unwrap();

        for i in 0..channelizers.len() {
            for j in i..channelizers.len() {
                if true {
                    correlator.correlate(&channelizers[i], &channelizers[j], &mut corr_data);
                    let fname = format!("{out_prefix}_{}{}.dat", i, j);
                    let mut outfile = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .write(true)
                        .open(&fname)
                        .unwrap();
                    //let mut outfile = std::fs::File::create(fname).unwrap();
                    write_data(&mut outfile, &corr_data);
                }
            }
        }
        idx += 1;
        if cfg.cnt>0 && idx>=cfg.cnt{
            running.fetch_not(Ordering::SeqCst);
            break;
        }
    }

    //println!("finished");

    //for i in 0..cnt{
    //    println!("{}", i);
    //    queue.fetch();
    //}

    //recv_threads.into_iter().for_each(|x| x.join().unwrap());
}
