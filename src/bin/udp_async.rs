#![feature(async_closure)]

use chrono::prelude::*;
use tokio::{net::UdpSocket, runtime::Runtime};

use csp::{
    cfg::{NCH_PER_STREAM, NFRAME_PER_CORR},
    cspch::{calc_coeff, Correlator, CspChannelizer},
    data_frame::CorrDataQueue,
    utils::write_data,
};
use futures::future::{join_all, select_all};

use csp::data_frame::DbfDataFrame;
use std::{cell::RefCell, io::Result};
use std::{fs::OpenOptions, io::Write};

fn main() -> Result<()> {
    let addresses = vec!["192.168.1.31:4001", "192.168.1.71:4001"];
    let ndevs = addresses.len();

    let (corr_queue, receiver): (Vec<_>, Vec<_>) = (0..ndevs).map(|_| CorrDataQueue::new()).unzip();
    let corr_queue = corr_queue
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();
    let nfine_eff = 16;
    let nfine_full = nfine_eff * 2;
    let coeffs = calc_coeff(nfine_full, 32, 0.95);
    let out_prefix = "test";

    let _t = std::thread::spawn(move || {
        {
            let mut channelizers = (0..ndevs)
                .map(|_| CspChannelizer::new(NFRAME_PER_CORR, NCH_PER_STREAM, nfine_full, &coeffs))
                .collect::<Vec<_>>();
            let mut correlator =
                Correlator::new(NCH_PER_STREAM * nfine_eff, NFRAME_PER_CORR / nfine_full);
            let mut corr_data = vec![0f32; NCH_PER_STREAM * nfine_eff * 2];
            //let mut idx = 0;
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
                .zip(receiver.iter().zip(channelizers.iter_mut()))
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
                //.write(true)
                .open(time_filename)
                .unwrap();

            writeln!(&mut time_file, "{}", time_i64).unwrap();

            let time_filename = format!("{out_prefix}_time.bin");
            let mut time_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(time_filename)
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
                            .open(&fname)
                            .unwrap();
                        //let mut outfile = std::fs::File::create(fname).unwrap();
                        write_data(&mut outfile, &corr_data);
                    }
                }
            }
            //idx += 1;
        }
    });

    let rt = Runtime::new()?;

    rt.block_on(async {
        let sockets = join_all(addresses.into_iter().map(UdpSocket::bind))
            .await
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        //let mut last_pkt_id = 0;

        let capturers = sockets
            .iter()
            .zip(corr_queue.iter())
            .map(|(s, q)| {
                async || {
                    let mut data = DbfDataFrame::default();
                    let buf = unsafe {
                        std::slice::from_raw_parts_mut(
                            (&mut data) as *mut DbfDataFrame as *mut u8,
                            8080,
                        )
                    };
                    let (_size, _addr) = s.recv_from(buf).await.unwrap();
                    //println!("{}", data.pkt_id);
                    q.borrow_mut().push(&data);
                    //println!("{} {}", addr, data.pkt_id);
                    data.pkt_id
                }
            })
            .collect::<Vec<_>>();

        loop {
            let f = capturers.iter().map(|x| Box::pin(x()));
            select_all(f).await;
        }

        //xx().await;
        //
    });
    Ok(())
}
