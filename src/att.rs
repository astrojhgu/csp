use crate::cfg::{MAX_FREQ, NCH_TOTAL, NPORTS_PER_STATION};
use futures::stream::AbortRegistration;
use ndarray::{s, Array2, ArrayView2, Axis};
use num_complex::Complex;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct AttCfg {
    pub value: Vec<u16>,
}

impl Default for AttCfg {
    fn default() -> AttCfg {
        AttCfg {
            value: vec![0; NPORTS_PER_STATION],
        }
    }
}

impl AttCfg {
    pub fn write<W: Write>(&self, w: &mut W) {
        w.write_all(unsafe {
            std::slice::from_raw_parts(self.value.as_ptr() as *const u8, NPORTS_PER_STATION * 2)
        })
        .unwrap();
    }
}
