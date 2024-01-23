use std::io::{Read, Write};

use crate::cfg::{NBEAMS, NCH_TOTAL, NPORTS_PER_STATION, NPORTS_TOTAL};
use ndarray::{s, Array3, ArrayView1};
use num_complex::Complex;

#[derive(Clone)]
pub struct DbfCoeffs(pub Array3<Complex<i16>>);

impl DbfCoeffs {
    pub fn new(data: &[Complex<i16>]) -> Self {
        let data = ArrayView1::from(data)
            .into_shape((NCH_TOTAL, NPORTS_TOTAL, NBEAMS))
            .expect("unable to reshape")
            .to_owned();
        Self(data)
    }

    pub fn get_wgt(
        self,
        board_id: usize,
        port_id: usize,
        ch: usize,
        beam_id: usize,
    ) -> Complex<i16> {
        let pid = board_id * NPORTS_PER_STATION + port_id;
        self.0[(ch, pid, beam_id)]
    }

    pub fn set_wgt(
        &mut self,
        x: Complex<i16>,
        board_id: usize,
        port_id: usize,
        ch: usize,
        beam_id: usize,
    ) {
        let pid = board_id * NPORTS_PER_STATION + port_id;
        self.0[(ch, pid, beam_id)] = x;
    }

    pub fn read_from<R: Read>(r: &mut R) -> Self {
        let mut data = vec![Complex::<i16>::default(); NCH_TOTAL * NPORTS_TOTAL * NBEAMS];
        let data_slice = unsafe {
            std::slice::from_raw_parts_mut(
                data.as_mut_ptr() as *mut u8,
                data.len() * std::mem::size_of::<Complex<i16>>(),
            )
        };
        r.read_exact(data_slice).unwrap();
        let data: Vec<_> = data
            .iter()
            .map(|&x| Complex::new(i16::from_be(x.re), i16::from_be(x.im)))
            .collect();
        Self::new(&data)
    }

    pub fn write_to<W: Write>(&self, w: &mut W) {
        let data = self
            .0
            .as_slice()
            .unwrap()
            .iter()
            .map(|&x| Complex::<i16>::new(x.re.to_be(), x.im.to_be()))
            .collect::<Vec<_>>();
        let x = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<Complex<i16>>(),
            )
        };
        w.write_all(x).unwrap()
    }

    pub fn reverse_phase(&mut self) {
        self.0.iter_mut().for_each(|x| {
            x.im = -x.im;
        });
    }
}

impl Default for DbfCoeffs {
    fn default() -> Self {
        let data = Array3::ones((NCH_TOTAL, NPORTS_TOTAL, NBEAMS)) * i16::MAX;
        Self(data)
    }
}

pub struct DelayCfg {
    pub delays: Vec<f32>,
}
