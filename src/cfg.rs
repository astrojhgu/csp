pub const NCH_PER_STREAM: usize = 100;
pub const NFRAME_PER_PKT: usize = 20;
pub type RawDataType = i16;
pub const SAMPLE_RATE: f32 = 480e6; //Sps
pub const MAX_FREQ: f32 = 240e6; //Hz
pub const NPKT_PER_CORR: usize = (1 << 16) / 2; // 1/(1/2/480e6*1024)
pub const NFRAME_PER_CORR: usize = NPKT_PER_CORR * NFRAME_PER_PKT;
pub const NCH_TOTAL: usize = 512;
pub const NBEAMS: usize = 2;
pub const NPORTS_TOTAL: usize = 16;
pub const NPORTS_PER_STATION: usize = 8;
pub const SMP_LEN: usize = 1024;
pub const PKT_LEN: usize = std::mem::size_of::<crate::data_frame::DbfDataFrame>();