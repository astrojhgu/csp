#![allow(non_snake_case)]
use binrw::{
    binrw,
    io::{Cursor, Read, Seek, Write},
    BinRead, BinResult, BinWrite,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::num::Wrapping;

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum MsgContent {
    #[bw(magic = 0x1001u16)]
    #[br(magic = 0x1001u16)]
    WorkMode(u32),

    #[bw(magic = 0x1002u16)]
    #[br(magic = 0x1002u16)]
    Status([u32; 2]),

    #[bw(magic = 0x1007u16)]
    #[br(magic = 0x1007u16)]
    Reset(u32),

    #[bw(magic = 0x1004u16)]
    #[br(magic = 0x1004u16)]
    SelfTest { magic: [u32; 5], action: u32 },

    #[bw(magic = 0x1008u16)]
    #[br(magic = 0x1008u16)]
    AcquireSample { acq_ad: u32, acq_ch: u32, ch: u32 },

    #[bw(magic = 0x1009u16)]
    #[br(magic = 0x1009u16)]
    BitCut {
        board_id: u32,
        filter_cut: u32,
        dbf_cut: u32,
    },

    #[bw(magic = 0x100au16)]
    #[br(magic = 0x100au16)]
    BaseCh {
        board_id: u32,
        beam1: [u32; 4],
        beam2: [u32; 4],
    },

    #[bw(magic = 0x100bu16)]
    #[br(magic = 0x100bu16)]
    BeamCtrl {
        beam_id: [u32; 4],
        calc_wgt: u32,
        apply_wgt: u32,
    },

    #[bw(magic = 0x1011u16)]
    #[br(magic = 0x1011u16)]
    MultiCtrl { cmd: u32, param: [u32; 3] },

    #[bw(magic = 0x3201u16)]
    #[br(magic = 0x3201u16)]
    AttSet([u32; 16]),
}

impl MsgContent {
    pub fn to_msg_type_code(&self) -> u16 {
        use MsgContent::*;
        match self {
            WorkMode { .. } => 0x1001,
            Status { .. } => 0x1002,
            Reset { .. } => 0x1007,
            SelfTest { .. } => 0x1004,
            AcquireSample { .. } => 0x1008,
            BitCut { .. } => 0x1009,
            BaseCh { .. } => 0x100a,
            BeamCtrl { .. } => 0x100b,
            MultiCtrl { .. } => 0x1011,
            AttSet { .. } => 0x3201,
        }
    }

    pub fn to_bytes(&self) -> (u16, Vec<u8>) {
        let mut data = Cursor::new(Vec::new());
        self.write(&mut data).unwrap();
        let result = data.into_inner();
        let payload = std::vec::Vec::from_iter(result.iter().copied().skip(2));
        let msg_code: u16 = ((result[0] as u16) << 8) + result[1] as u16;
        (msg_code, payload)
    }

    pub fn verify_reply(&self, buf: &[u8]) -> bool {
        use MsgContent::*;
        let reply_msg: Value =
            from_str(std::str::from_utf8(buf).expect("failed to decode")).expect("failed to parse");

        match self {
            WorkMode { .. } => {
                reply_msg["msg_type"].as_str().expect("failed to parse") == "WorkMode"
            } //
            Status { .. } => reply_msg["msg_type"].as_str().expect("failed to parse") == "Status", //
            Reset { .. } => reply_msg["msg_type"].as_str().expect("failed to parse") == "Reset", //
            SelfTest { .. } => {
                reply_msg["msg_type"].as_str().expect("failed to parse") == "SelfTest"
            } //
            AcquireSample { .. } => {
                reply_msg["msg_type"].as_str().expect("failed to parse") == "AcquireSample"
            }
            BitCut { .. } => reply_msg["msg_type"].as_str().expect("failed to parse") == "BitCut",
            BaseCh { .. } => reply_msg["msg_type"].as_str().expect("failed to parse") == "BaseCh", //
            BeamCtrl { .. } => {
                reply_msg["msg_type"].as_str().expect("failed to parse") == "BeamCtrl"
            } //
            MultiCtrl { .. } => match reply_msg["msg_type"].as_str().expect("failed to parse") {
                "MultiCtrl" => true,
                "IPAddr" => true,
                _ => false,
            },
            AttSet { .. } => reply_msg["msg_type"].as_str().expect("failed to parse") == "AttSet", //
        }
    }

    pub fn show_bytes(&self) {
        let mut buf = Cursor::new(Vec::new());
        self.write(&mut buf).unwrap();
        let buf = buf.into_inner();
        println!("{:02x} {:02x}", buf[0], buf[1]);
        buf[2..].chunks(4).enumerate().for_each(|(i, x)| {
            print!("{i:04} ");
            x.iter().for_each(|&x1| {
                print!("{x1:02x} ");
            });
            println!();
        });
    }

    /*
    pub fn from_msg_type_code(code: u16) -> MsgType {
        use MsgType::*;
        match code {
            0x1001 => WorkMode,
            0x1002 => Status,
            0x1004 => SelfTest,
            0x1008 => AcquireSample,
            0x1009 => BitCut,
            0x100a => BaseCh,
            0x100b => BeamCtrl,
            0x1011 => MultiCtrl,
            0x2002 => Sync,
            0x3201 => AttSet,
            _ => panic!("invalid type code"),
        }
    }*/
}

#[derive(Debug)]
#[binrw]
#[brw(big)]

pub struct MsgHeader {
    pub header: u32,    //4
    pub pkt_len: u32,   //8
    pub rev1: [u8; 6],  //14
    pub msg_type: u16,  //16
    pub timestamp: u32, //20
    pub microsecs: u32, //24
    pub msg_cnt: u16,   //26
    pub rev2: [u8; 6],  //32
}

impl MsgHeader {
    pub fn new(msg_type: u16, pkt_len: usize, msg_cnt: u16) -> Self {
        let t_us = chrono::offset::Utc::now().timestamp_micros();
        let t_s = t_us / 1_000_000;
        let t_sub_sec_us = t_us - t_s * 1_000_000;
        MsgHeader {
            header: 0x5a5a5a5a,
            pkt_len: pkt_len as u32,
            rev1: [0_u8; 6],
            msg_type,
            timestamp: t_s as u32,
            microsecs: t_sub_sec_us as u32,
            msg_cnt,
            rev2: [0_u8; 6],
        }
    }

    pub fn show_bytes(&self) {
        let mut buf = Cursor::new(Vec::new());
        self.write(&mut buf).unwrap();
        let buf = buf.into_inner();
        //println!("{:02x} {:02x}", buf[0], buf[1]);
        buf.chunks(4).enumerate().for_each(|(i, x)| {
            print!("{i:04} ");
            x.iter().for_each(|&x1| {
                print!("{x1:02x} ");
            });
            println!();
        });
    }
}

#[derive(Debug)]
#[binrw]
#[brw(big)]
pub struct MsgTail {
    pub rev: [u8; 3],
    pub checksum: u8,
    pub tail: [u8; 8],
}

impl MsgTail {
    pub fn show_bytes(&self) {
        let mut buf = Cursor::new(Vec::new());
        self.write(&mut buf).unwrap();
        let buf = buf.into_inner();
        //println!("{:02x} {:02x}", buf[0], buf[1]);
        buf.chunks(4).enumerate().for_each(|(i, x)| {
            print!("{i:04} ");
            x.iter().for_each(|&x1| {
                print!("{x1:02x} ");
            });
            println!();
        });
    }
}

pub struct CtrlMsg {
    pub head: MsgHeader,
    pub payload: Vec<u8>,
    pub tail: MsgTail,
}

impl CtrlMsg {
    pub fn new(msg_cont: MsgContent, msg_cnt: u16) -> Self {
        let (code, payload) = msg_cont.to_bytes();
        let head = MsgHeader::new(code, Self::calc_msg_len(payload.len()) as usize, msg_cnt);
        let checksum = Self::calc_checksum(&head, &payload);
        CtrlMsg {
            head,
            payload,
            tail: MsgTail {
                rev: [0_u8; 3],
                checksum,
                tail: [0xaa; 8],
            },
        }
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.head.write(writer)?;
        writer.write_all(&self.payload)?;
        self.tail.write(writer)?;
        BinResult::Ok(())
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let head = MsgHeader::read(reader)?;
        //let msg_type=head.msg_type;
        //let mut buf = Vec::<u8>::new();
        let msg_len = head.pkt_len as usize;
        let payload_len =
            msg_len - (std::mem::size_of::<MsgHeader>() + std::mem::size_of::<MsgTail>());
        let mut payload = vec![0u8; payload_len];
        reader.read_exact(&mut payload)?;
        let tail = MsgTail::read(reader)?;
        Ok(Self {
            head,
            payload,
            tail,
        })
    }

    pub fn show_bytes(&self) {
        self.head.show_bytes();
        self.get_payload().unwrap().show_bytes();
        self.tail.show_bytes();
    }

    pub fn get_payload(&self) -> BinResult<MsgContent> {
        let msg_type = self.head.msg_type;

        let x: [u8; 2] = msg_type.to_be_bytes();
        let mut buf = vec![x[0], x[1]];
        self.payload.iter().for_each(|&x| buf.push(x));

        let mut buf = Cursor::new(buf);
        MsgContent::read(&mut buf)
    }

    pub fn calc_msg_len(payload_len: usize) -> u32 {
        (std::mem::size_of::<MsgHeader>() + std::mem::size_of::<MsgTail>() + payload_len) as u32
    }

    fn calc_checksum(head: &MsgHeader, payload: &[u8]) -> u8 {
        let mut checksum = std::num::Wrapping(0_u8);
        let mut buf = Cursor::new(Vec::new());
        head.write(&mut buf).unwrap();
        payload
            .iter()
            .copied()
            .chain([0_u8; 3].into_iter())
            .for_each(|x| buf.get_mut().push(x));

        for x in buf.into_inner().into_iter().skip(4) {
            checksum -= Wrapping(x);
        }
        checksum.0
    }
}
