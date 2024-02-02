use lockfree_object_pool::{LinearObjectPool, LinearOwnedReusable};
use std::{default::Default, sync::Arc};

use std::io::Write;

use byteorder::{ByteOrder, LittleEndian};

use crossbeam::channel::{bounded, Receiver, Sender};

use crate::cfg::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DbfDataFrame {
    //time domain data
    pub head1: u64,
    pub head2: u64,
    pub head3: u64,
    pub head4: u64,
    pub date: u64,
    pub subsec: u64,
    pub beam_id: u64,
    pub pkt_id: u64,
    pub start: u64,
    pub payload: [i16; NCH_PER_STREAM * 2 * NFRAME_PER_PKT],
    pub tail: u64,
}

impl Default for DbfDataFrame {
    fn default() -> Self {
        DbfDataFrame {
            head1: 0,
            head2: 0,
            head3: 0,
            head4: 0,
            date: 0,
            subsec: 0,
            beam_id: 0,
            pkt_id: 0,
            start: 0,
            payload: [RawDataType::default(); NCH_PER_STREAM * 2 * NFRAME_PER_PKT],
            tail: 0,
        }
    }
}

impl DbfDataFrame {
    pub fn from_raw(src: &[u8]) -> Self {
        assert_eq!(src.len(), std::mem::size_of::<DbfDataFrame>());

        let mut result = DbfDataFrame::default();
        let ptr_head = unsafe {
            std::slice::from_raw_parts_mut((&mut result) as *mut DbfDataFrame as *mut u64, 9)
        };

        LittleEndian::read_u64_into(&src[..72], ptr_head);
        //result.pkt_id=result.pkt_id.swap_bytes();
        let payload_begin = 72;
        let payload_end =
            72 + NCH_PER_STREAM * 2 * NFRAME_PER_PKT * std::mem::size_of::<RawDataType>();
        LittleEndian::read_i16_into(&src[payload_begin..payload_end], &mut result.payload);

        result.tail = LittleEndian::read_u64(&src[payload_end..]);

        assert_eq!(result.head1, 0xa5a5_a5a5_a5a5_a5a5_u64);
        assert_eq!(result.head2, 0xa5a5_a5a5_a5a5_a5a5_u64);
        assert_eq!(result.head3, 0xbc55_0000_bc55_0000_u64);
        assert_eq!(result.head4, 0xeeee_0000_0000_0000_u64);
        assert_eq!(result.start, 0x7777_7777_7777_7777_u64);
        assert_eq!(result.tail, 0xbcbc_bcbc_bcbc_bcbc_u64);
        result
    }

    pub fn write_payload<W>(&self, out: &mut W)
    where
        W: Write,
    {
        let data = unsafe {
            std::slice::from_raw_parts(
                self.payload.as_ptr() as *const u8,
                NCH_PER_STREAM * 2 * NFRAME_PER_PKT * std::mem::size_of::<RawDataType>(),
            )
        };
        out.write_all(data).unwrap();
    }
}

pub struct CorrDataFrame {
    pub corr_id: usize,
    pub payload: Vec<RawDataType>,
}

impl Default for CorrDataFrame {
    fn default() -> Self {
        Self {
            corr_id: 0,
            payload: vec![
                RawDataType::default();
                NCH_PER_STREAM * 2 * NFRAME_PER_PKT * NPKT_PER_CORR
            ],
        }
    }
}

impl CorrDataFrame {
    pub fn fill(&mut self, pkt: &DbfDataFrame) -> usize {
        let pkt_id = pkt.pkt_id;
        let corr_id = pkt_id as usize / NPKT_PER_CORR;
        let offset =
            (pkt_id as usize - corr_id * NPKT_PER_CORR) * NCH_PER_STREAM * 2 * NFRAME_PER_PKT;
        self.payload[offset..offset + NCH_PER_STREAM * 2 * NFRAME_PER_PKT]
            .copy_from_slice(&pkt.payload);
        self.corr_id = corr_id;

        //println!("{} {}", offset, self.payload.len());
        offset
    }
    pub fn clear(&mut self) {
        self.corr_id = 0;
        self.payload.fill(0);
    }

    pub fn write_payload<W>(&self, out: &mut W)
    where
        W: Write,
    {
        let data = unsafe {
            std::slice::from_raw_parts(
                self.payload.as_ptr() as *const u8,
                NCH_PER_STREAM//100
                    * 2 //re and im
                    * NFRAME_PER_PKT //20
                    * NPKT_PER_CORR  //integration time
                    * std::mem::size_of::<RawDataType>(),
            )
        };
        out.write_all(data).unwrap();
    }
}

pub struct CorrDataQueue {
    pub last_pkt_id: Option<usize>,
    pub pool: Arc<LinearObjectPool<CorrDataFrame>>,
    pub tmp_corr_data_frame: LinearOwnedReusable<CorrDataFrame>,
    pub sender: Sender<LinearOwnedReusable<CorrDataFrame>>,
}

impl CorrDataQueue {
    pub fn new() -> (Self, Receiver<LinearOwnedReusable<CorrDataFrame>>) {
        let (sender, receiver) = bounded(16);
        let pool = Arc::new(LinearObjectPool::new(
            || {
                println!("initialized");
                CorrDataFrame::default()
            },
            |v| {
                //println!("reseted");
                v.corr_id = 0;
                v.payload.fill(RawDataType::default());
            },
        ));
        let tmp_corr_data_frame = pool.pull_owned();
        //let tmp=pool.pull(||CorrDataFrame::default());
        let result = Self {
            last_pkt_id: None,
            pool,
            tmp_corr_data_frame,
            sender,
        };
        (result, receiver)
    }

    pub fn push(&mut self, pkt: &DbfDataFrame) {
        let pkt_id = pkt.pkt_id as usize;
        let frame_id = pkt_id / NPKT_PER_CORR;
        //println!("{}", frame_id);
        if let Some(last_pkt_id) = self.last_pkt_id {
            if last_pkt_id + 1 != pkt.pkt_id as usize {
                eprintln!(
                    "dropped {} packets {} {}",
                    pkt.pkt_id as i64 - last_pkt_id as i64 - 1, pkt.pkt_id, last_pkt_id
                );
            }
        }
        self.last_pkt_id = Some(pkt.pkt_id as usize);
        let _offset = self.tmp_corr_data_frame.fill(pkt);
        let next_frame_id = (pkt_id + 1) / NPKT_PER_CORR;
        if next_frame_id != frame_id {
            println!("swapped {} {}", frame_id, pkt.pkt_id);
            //println!("")
            let result = std::mem::replace(&mut self.tmp_corr_data_frame, self.pool.pull_owned());
            match self.sender.send(result) {
                Ok(_) => {}
                Err(e) => {
                    panic!("send error {:?}", e);
                }
            }
        }
        
        //println!("{:?}", &self.tmp_corr_data_frame.payload[offset..offset+32]);
    }
}
