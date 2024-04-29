use std::{io::Read, fs::File};
use clap::Parser;
use csp::att::AttCfg;
use suppaftp::FtpStream;
use serde_yaml::from_reader;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('i'), long("in"), value_name = "input wgt file")]
    infile: String,

    #[clap(short('a'), long("addr"), value_name = "<IPv4 address>")]
    addr: String,
}

fn main(){
    let args=Args::parse();
    let addr=format!("{}:21", args.addr);
    let mut ftp_stream=FtpStream::connect(addr).unwrap();
    ftp_stream.login("", "").unwrap();
    
    let att:AttCfg=from_reader(File::open(args.infile).unwrap()).unwrap();

    let dest_att_file="/ata0:0/config/AttCtrl.dat";
    let mut stream=ftp_stream.put_with_stream(dest_att_file).unwrap();
    att.write(&mut stream);
    ftp_stream.finalize_put_stream(stream).unwrap();
}
