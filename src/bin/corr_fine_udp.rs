use clap::Parser;

use csp::{
    cfg::{NCH_PER_STREAM, NFRAME_PER_CORR},
    cspch::{calc_coeff, Correlator, CspChannelizer},
    data_frame::{CorrDataQueue, DbfDataFrame},
    utils::write_data,
};

use std::{
    fs::OpenOptions,
    io::Write,
    net::{SocketAddr, SocketAddrV4, UdpSocket},
};

use chrono::prelude::*;

const PKT_LEN: usize = 8080;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'd', long = "addr", num_args(1..), value_name="ip:port")]
    dev: Vec<String>,

    #[clap(short = 'o', long = "out", value_name = "out prefix")]
    out_prefix: String,

    /// If dry run
    #[clap(short('c'), long("cnt"), value_name = "cnt", default_value_t = 0)]
    cnt: usize,

    #[clap(short('f'), long("fine"), value_name = "num fine ch")]
    nfine: usize,

    #[clap(
        short('t'),
        long("tap"),
        value_name = "tap per fine ch",
        default_value_t = 8
    )]
    tap_per_fine_ch: usize,

    #[clap(
        short('k'),
        long("k"),
        value_name = "filter coeff k",
        default_value_t = 0.8
    )]
    k: f32,
}

fn main() {
    let args = Args::parse();

    let src_ips = args.dev;
    let mut src_addrs = src_ips
        .iter()
        .map(|s| s.parse::<SocketAddrV4>().unwrap())
        .collect::<Vec<_>>();
    src_addrs.sort();
    let src_addrs = src_addrs;
    println!("{:?}", src_addrs);
    //std::process::exit(0);

    println!("{src_ips:?}");

    let nfine_eff = args.nfine;
    let nfine_full = nfine_eff * 2;

    let coeffs = calc_coeff(nfine_full, args.tap_per_fine_ch, args.k);
    let mut channelizers = src_ips
        .iter()
        .map(|_| CspChannelizer::new(NFRAME_PER_CORR, NCH_PER_STREAM, nfine_full, &coeffs))
        .collect::<Vec<_>>();

    let out_prefix = args.out_prefix;

    let cnt = args.cnt;

    let n_stations = src_ips.len();
    let (mut corr_queue, receiver): (Vec<_>, Vec<_>) =
        (0..n_stations).map(|_| CorrDataQueue::new()).unzip();

    std::thread::spawn(move || {
        let udp_socket = UdpSocket::bind("0.0.0.0:4001").unwrap();
        let mut old_pkt_id = 0;
        let mut pkt_cnt = 0;
        let mut data = DbfDataFrame::default();
        let buf = unsafe {
            std::slice::from_raw_parts_mut((&mut data) as *mut DbfDataFrame as *mut u8, 8080)
        };
        loop {
            let (len, src_addr) = udp_socket.recv_from(buf).unwrap();
            if len != PKT_LEN {
                continue;
            }

            //println!("src_addr:{}", src_addr);
            match src_addr {
                SocketAddr::V4(s) => match src_addrs.binary_search(&s) {
                    Ok(i) => {
                        corr_queue[i].push(&data)},
                    Err(_) => {
                        panic!("unregistered station addr");
                    }
                },
                SocketAddr::V6(_s) => {
                    continue},
            }
        }
    });

    let mut channelized_data = vec![0_f32; channelizers[0].output_buf_len()];
    let mut correlator = Correlator::new(NCH_PER_STREAM * nfine_eff, NFRAME_PER_CORR / nfine_full);
    let mut corr_data = vec![0f32; NCH_PER_STREAM * nfine_eff * 2];
    let mut idx = 0;
    while cnt == 0 || idx < cnt {
        let mut corr_id_list = Vec::new();
        let mut max_corr_id = 0;
        receiver
            .iter()
            .zip(channelizers.iter_mut())
            .for_each(|(r, channelizer1)| {
                let x = r.recv().unwrap();
                println!("{} {}", x.corr_id, receiver.len());
                corr_id_list.push(x.corr_id);
                max_corr_id = max_corr_id.max(x.corr_id);
                //channelizer1.channelize(&x.payload, &mut channelized_data);
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
    }

    println!("finished");

    //for i in 0..cnt{
    //    println!("{}", i);
    //    queue.fetch();
    //}

    //recv_threads.into_iter().for_each(|x| x.join().unwrap());
}
