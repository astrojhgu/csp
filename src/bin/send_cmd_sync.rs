use clap::Parser;
use serde_yaml::from_reader;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    net::UdpSocket,
    sync::Arc,
    time::Duration,
};
//use dbf_ctrl::data_frame::{CorrDataQueue, DbfDataFrame};
use csp::ctrl_msg::{CtrlMsg, MsgContent};

use binrw::io::Cursor;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config
    #[clap(short = 'a', long = "addr", num_args(1..), value_name="ip:port")]
    addr: String,

    #[clap(short = 'p', long = "port", value_name = "local port")]
    port: u16,

    #[clap(short = 'c', long = "cmd", value_name = "cmd.yaml")]
    cmd: String,

    #[clap(short = 't', long = "timeout", value_name = "timeout in sec")]
    t: Option<f64>,
}

pub fn main() {
    let args = Args::parse();
    //let msg_content = MsgContent::AttSet([0; 16]);

    //socket.set_read_timeout(args.t.map(|t| Duration::from_secs_f64(t)));

    let mut recv_buf = vec![0_u8; 9000];
    let msg_contents: Vec<MsgContent> =
        from_reader(File::open(&args.cmd).expect("file not opened"))
            .expect("message cannot be loaded");

    let nmsgs = msg_contents.len();
    for (i, msg_content) in msg_contents.into_iter().enumerate() {
        let local_addr = format!("0.0.0.0:{}", args.port);
        let socket = Arc::new(UdpSocket::bind(&local_addr).unwrap());
        socket.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
        socket.set_broadcast(true).expect("broadcast set failed");
        socket.set_nonblocking(false).expect("nonblocking set failed");

        let msg = CtrlMsg::new(msg_content, i as u16 + 1);
        //msg.show_bytes();
        let mut buf = Cursor::new(Vec::new());
        msg.write(&mut buf).expect("msg write failed");
        let buf = buf.into_inner();

        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("sent_bytes.txt")
            .unwrap();

        for w in buf.as_slice().chunks(4) {
            for &b in w{
                write!(&mut log_file, "{b:02x} ").unwrap();
            }
            writeln!(&mut log_file).unwrap();
        }
        writeln!(&mut log_file,"\n").unwrap();

        socket.send_to(&buf, &args.addr).expect("send err");

        loop {
            if let Ok((num_of_bytes, src_addr)) = socket.recv_from(&mut recv_buf) {
                println!("{num_of_bytes} received from {src_addr:?}");
                let reply =
                    std::str::from_utf8(&recv_buf[..num_of_bytes]).expect("failed to decode");
                println!("Reply: {reply}");
                if msg_content.verify_reply(&recv_buf[..num_of_bytes]) {
                    println!("done");
                    break;
                } else {
                    println!("not yet");
                }
            } else {
                println!("terminated for timeout");
                break;
            }
        }

        if i != nmsgs - 1 {
            std::thread::sleep(Duration::from_secs_f64(args.t.unwrap_or(1.0)));
        }
    }
}
