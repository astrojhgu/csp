use csp::addr_cfg::{AddrCfg, FiberAddrPair};
use serde_yaml::to_writer;
use std::{fs::File, io::{Read, Write}, ops::Add};

fn main() {
    let mut raw_data=vec![0; 160];
    let mut infile=File::open("IPAddr.dat").unwrap();
    infile.read_exact(&mut raw_data).unwrap();

    let addr_cfg=AddrCfg::from_raw(&raw_data);
    println!("{:?}", addr_cfg);

    let outfile=File::create("ip_cfg.yaml").unwrap();
    to_writer(outfile, &addr_cfg).unwrap();
}
