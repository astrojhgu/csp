use clap::Parser;
use csp::wgt::{WgtCfg, WgtFlags};
use serde_yaml::{from_reader, to_writer};
use std::fs::File;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('c'), long("cfg"), value_name = "cfg file")]
    cfg_file: String,

    #[clap(short('o'), long("out"), value_name = "output file")]
    out_file: String,
}

fn main() {
    let args:Args=Args::parse();
    
    let wgt_cfg:WgtCfg=from_reader(File::open(&args.cfg_file).unwrap()).unwrap();
    println!("{:?}", wgt_cfg);

    let wgt=wgt_cfg.calc();
    println!("{:?}", wgt);
    /*
    let wgt_cfg=WgtCfg{
        delay: vec![0.0; 128],
        gain: vec![0.0; 128], 
        flags: WgtFlags::None
    };

    to_writer(File::create("a.yaml").unwrap(), &wgt_cfg).unwrap();
    */
}
