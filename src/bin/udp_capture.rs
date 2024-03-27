use std::net::UdpSocket;

use csp::data_frame::DbfDataFrame;

fn main() {
    let current_id = std::sync::Arc::new(std::sync::Mutex::new(15));
    let get_capturer = |addr: &str, blocking: bool| {
        let addr = addr.to_string();
        let lock = std::sync::Arc::clone(&current_id);
        move || {
            {
                let mut id = lock.lock().unwrap();
                //set_for_current(CoreId { id: *id });
                *id += 1;
            }

            println!("capturing from {}", addr);
            let socket = UdpSocket::bind(&addr).unwrap();
            socket.set_nonblocking(!blocking).unwrap();

            let mut data = DbfDataFrame::default();
            let buf = unsafe {
                std::slice::from_raw_parts_mut((&mut data) as *mut DbfDataFrame as *mut u8, 8080)
            };
            let mut n;

            let (_amt, _src) = loop {
                if let Ok((amt, src)) = socket.recv_from(buf) {
                    break (amt, src);
                }
            };

            let mut npkt: u64 = 0;
            let mut npkt_dropped: u64 = 1;

            n = data.pkt_id;
            loop {
                let (amt, src) = loop {
                    if let Ok((amt, src)) = socket.recv_from(buf) {
                        break (amt, src);
                    }
                };
                if amt != 8080 {
                    continue;
                }

                if data.pkt_id != n + 1 {
                    let d = data.pkt_id - n - 1;
                    npkt_dropped += d;
                    println!(
                        "{}: {} pkt dropped, ratio={:e}, cnt={}, prev cnt={}",
                        addr,
                        d,
                        1.0 / ((npkt / npkt_dropped) as f64),
                        data.pkt_id,
                        n
                    );
                }
                npkt += 1;
                if npkt % 65536 == 0 {
                    println!("{} {:e}", src, 1.0 / ((npkt / npkt_dropped) as f64));
                }
                assert!(n < data.pkt_id);
                n = data.pkt_id;
            }
        }
    };

    let th1 = std::thread::spawn(get_capturer("192.168.1.71:4001", true));

    let th2 = std::thread::spawn(get_capturer("192.168.1.31:4001", true));

    th1.join().unwrap();

    th2.join().unwrap();
}
