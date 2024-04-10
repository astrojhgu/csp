use std::{fs::File, io::Write};
use clap::Parser;
use csp::addr_cfg::AddrCfg;
use serde_yaml::from_reader;
use suppaftp::FtpStream;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('i'), long("in"), value_name = "input ip cfg.yaml")]
    infile: String,

    #[clap(short('a'), long("addr"), value_name = "<IPv4 address>")]
    addr: String,
}

fn main(){
    let args=Args::parse();
    let addr=format!("{}:21", args.addr);
    let mut ftp_stream=FtpStream::connect(addr).unwrap();
    ftp_stream.login("", "").unwrap();

    let addr_cfg:AddrCfg=from_reader(File::open(args.infile).unwrap()).unwrap();
    let raw_data=addr_cfg.to_raw();
    let dest_ip_file="/ata0:0/config/IPAddr.dat";
    let mut stream=ftp_stream.put_with_stream(dest_ip_file).unwrap();
    stream.write_all(&raw_data).unwrap();
    ftp_stream.finalize_put_stream(stream).unwrap();
}
