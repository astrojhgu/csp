use clap::Parser;

use csp::{
    cfg::{NCH_PER_STREAM, NFRAME_PER_CORR},
    cspch::{calc_coeff, Correlator, CspChannelizer},
    data_frame::{CorrDataQueue, DbfDataFrame},
    utils::write_data,
};

use std::{io::Write, fs::OpenOptions};

use chrono::prelude::*;

const PKT_LEN: usize = 8080;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'd', long = "dev", num_args(1..), value_name="devs")]
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

    let dev_name = args.dev;
    println!("{dev_name:?}");

    let nfine_eff = args.nfine;
    let nfine_full = nfine_eff * 2;

    let coeffs = calc_coeff(nfine_full, args.tap_per_fine_ch, args.k);
    let mut channelizers = dev_name
        .iter()
        .map(|_| CspChannelizer::new(NFRAME_PER_CORR, NCH_PER_STREAM, nfine_full, &coeffs))
        .collect::<Vec<_>>();

    let out_prefix = args.out_prefix;

    let cnt = args.cnt;

    let ndevs = dev_name.len();
    let (corr_queue, receiver): (Vec<_>, Vec<_>) = (0..ndevs).map(|_| CorrDataQueue::new()).unzip();

    let _recv_threads: Vec<_> = dev_name
        .iter()
        .zip(corr_queue.into_iter())
        .map(|(dvn, mut sender)| {
            let device = pcap::Device::list()
                .unwrap()
                .iter()
                .find(|&d| d.name == *dvn)
                .unwrap()
                .clone();
            println!("Using device {}", device.name);

            let cap = pcap::Capture::from_device(device)
                .unwrap()
                .immediate_mode(false)
                .buffer_size(1024 * 1024 * 1024)
                .promisc(true)
                .timeout(0);

            let mut cap = cap.open().unwrap();
            cap.direction(pcap::Direction::In).unwrap();

            let _dvn = dvn.clone();
            std::thread::spawn(move || {
                let mut old_pkt_id = 0;
                let mut pkt_cnt = 0;
                loop {
                    match cap.next_packet() {
                        Ok(pkt) if pkt.data.len() == PKT_LEN + 42 => {
                            let frame_buf1 = DbfDataFrame::from_raw(&pkt.data[42..]); //skip udp head
                            let pkt_id = frame_buf1.pkt_id as usize;
                            //println!("{} {}",dvn, pkt_id);
                            if pkt_cnt == 0 {
                                println!("c0={pkt_id}",);
                            } else if old_pkt_id + 1 != pkt_id {
                                println!("dropped {} pkts", pkt_id - old_pkt_id - 1);
                            }

                            pkt_cnt += 1;
                            old_pkt_id = pkt_id;

                            sender.push(&frame_buf1);
                        }
                        Err(e) => println!("{e:?}"),
                        Ok(pkt) => {
                            println!("len: {}", pkt.data.len());
                        }
                    }
                }
            })
        })
        .collect();

    let mut channelized_data = vec![0_f32; channelizers[0].output_buf_len()];
    let mut correlator = Correlator::new(NCH_PER_STREAM * nfine_eff, NFRAME_PER_CORR / nfine_full);
    let mut corr_data = vec![0f32; NCH_PER_STREAM * nfine_eff * 2];
    let mut idx = 0;
    while cnt == 0 || idx < cnt {
        let mut corr_id_list = Vec::new();
        let mut max_corr_id = 0;
        receiver
            .iter()
            .zip(dev_name.iter().zip(channelizers.iter_mut()))
            .for_each(|(r, (_dn, channelizer1))| {
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
                    .zip(dev_name.iter().zip(channelizers.iter_mut())),
            )
            .for_each(|(&cid, (r, (_dn, channelizer1)))| {
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
        let time_i64=now.timestamp();
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

        let b=time_i64.to_le_bytes();
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
