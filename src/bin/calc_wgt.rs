use clap::Parser;
use csp::wgt::{WgtCfg, write_wgt};
use serde_yaml::from_reader;
use std::{fs::File, io::Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('c'), long("cfg"), value_name = "cfg file")]
    cfg_file: String,

    #[clap(short('o'), long("out"), value_name = "output file")]
    out_file: Option<String>,
}

fn main() {
    let args:Args=Args::parse();
    
    let wgt_cfg:WgtCfg=from_reader(File::open(&args.cfg_file).unwrap()).unwrap();
    println!("{:?}", wgt_cfg);

    let wgt=wgt_cfg.calc();
    let mut outfile:Box<dyn Write>=
    if let Some(ref fname)=args.out_file{
        Box::new(File::create(fname).unwrap())
    }else{
        Box::new(std::io::stdout())
    };
    
    write_wgt(&mut outfile, wgt.view());
}
