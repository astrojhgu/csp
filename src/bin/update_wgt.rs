use std::io::Write;

use suppaftp::FtpStream;

fn main(){
    let mut ftp_stream=FtpStream::connect("192.168.4.12:21").unwrap();
    let _ = ftp_stream.login("", "").unwrap();
    let mut stream=ftp_stream.put_with_stream("/ata0:0/config/a.bin").unwrap();
    stream.write(&[0,1,2,3,4,5,6,7]).unwrap();
    ftp_stream.finalize_put_stream(stream).unwrap();
}
