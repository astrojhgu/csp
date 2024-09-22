use std::{fs::File, io::{Read, Write}};
use clap::Parser;
use csp::wgt::{read_wgt, write_wgt};
use suppaftp::FtpStream;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short('i'), long("in"), value_name = "input wgt file")]
    infile: Option<String>,

    #[clap(short('a'), long("addr"), value_name = "<IPv4 address>")]
    addr: String,

    #[clap(short('b'), long("beam"), value_name = "beam id 0|1|2")]
    beam_id: u32
}

fn main(){
    let args=Args::parse();
    let addr=format!("{}:21", args.addr);
    let mut ftp_stream=FtpStream::connect(addr).unwrap();
    ftp_stream.login("", "").unwrap();
    
    
    let beam_id=if args.beam_id==0{
        vec![1,2]
    }else{
        assert!(args.beam_id==1 || args.beam_id==2);
        vec![args.beam_id]
    };

    let mut infile:Box<dyn Read>=if let Some(ref fname)=args.infile{
        Box::new(File::open(fname).unwrap())
    }else{
        Box::new(std::io::stdin())
    };


    let wgt=read_wgt(&mut infile);

    for b in beam_id{
        let dest_wgt_file=format!("/ata0:0/config/DbfInitCoeff{}.dat", b);
        ftp_stream.rm(&dest_wgt_file).unwrap();
        let mut stream=ftp_stream.put_with_stream(&dest_wgt_file).unwrap();
        write_wgt(&mut stream, wgt.view());
        stream.flush().unwrap();
        ftp_stream.finalize_put_stream(stream).unwrap();
    }
}
