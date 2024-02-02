use crate::cfg::{MAX_FREQ, NCH_TOTAL, NPORTS_PER_STATION};
use ndarray::{s, Array2, ArrayView2, Axis};
use num_complex::Complex;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub enum WgtFlags {
    Enable(Vec<usize>),
    Disable(Vec<usize>),
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WgtCfg {
    pub delay: Vec<f64>,
    pub ampl: Vec<f64>,
    pub flags: WgtFlags,
}

impl WgtCfg {
    pub fn calc(&self) -> Array2<Complex<f64>> {
        let delay = delay2wgt(&self.delay);
        let ampl = ampl2wgt(&self.ampl);
        let flags = match self.flags {
            WgtFlags::Enable(ref x) => enable_ports(x),
            WgtFlags::Disable(ref x) => disable_ports(x),
            WgtFlags::None => unit_wgt(),
        };
        let wgt = vec![delay, ampl, flags];
        merge_wgt(&wgt)
    }
}


pub fn write_wgt(writer: &mut dyn Write, data: ArrayView2<Complex<f64>>) {
    assert_eq!(data.nrows(), NCH_TOTAL);
    assert_eq!(data.ncols(), NPORTS_PER_STATION);
    data.as_slice().unwrap().iter().for_each(|x| {
        assert!(x.re.abs() <= 1.0);
        assert!(x.im.abs() <= 1.0);
        let data1 = [(32767.0*x.re).round() as i16, (32767.0*x.im).round() as i16];
        writer
            .write(unsafe { std::slice::from_raw_parts(data1.as_ptr() as *const u8, 4) })
            .unwrap();
    })
}

pub fn read_wgt(reader: &mut dyn Read) -> Array2<Complex<f64>> {
    let mut data = Array2::<Complex<i16>>::zeros((NCH_TOTAL, NPORTS_PER_STATION));
    {
        let buf = unsafe {
            std::slice::from_raw_parts_mut(
                data.as_mut_ptr() as *mut u8,
                NCH_TOTAL * NPORTS_PER_STATION * 2 * 2,
            )
        };
        reader.read(buf).unwrap();
    }
    data.map(|x| Complex::new(x.re as f64/32767.0, x.im as f64/32767.0))
}

pub fn merge_wgt(input: &[Array2<Complex<f64>>]) -> Array2<Complex<f64>> {
    let mut result = Array2::<Complex<f64>>::ones((NCH_TOTAL, NPORTS_PER_STATION));
    for x in input {
        result *= x;
    }
    result
}

pub fn unit_wgt() -> Array2<Complex<f64>> {
    Array2::<Complex<f64>>::ones((NCH_TOTAL, NPORTS_PER_STATION))
}

pub fn zero_wgt() -> Array2<Complex<f64>> {
    Array2::<Complex<f64>>::zeros((NCH_TOTAL, NPORTS_PER_STATION))
}

pub fn disable_ports(pid: &[usize]) -> Array2<Complex<f64>> {
    let mut result = unit_wgt();
    for &i in pid {
        result.slice_mut(s![.., i]).fill(Complex::zero());
    }
    result
}

pub fn enable_ports(pid: &[usize]) -> Array2<Complex<f64>> {
    let mut result = zero_wgt();
    for &i in pid {
        result.slice_mut(s![.., i]).fill(Complex::new(1.0, 0.0));
    }
    result
}

pub fn delay2wgt(delay_ns: &[f64]) -> Array2<Complex<f64>> {
    assert_eq!(delay_ns.len(), NPORTS_PER_STATION);
    let mut result = unit_wgt();
    for (ic, mut w_ch) in result.axis_iter_mut(Axis(0)).enumerate() {
        let freq = ic as f64 / NCH_TOTAL as f64 * MAX_FREQ as f64;
        for (&d_ns, w) in delay_ns.iter().zip(w_ch.iter_mut()) {
            *w *= Complex::new(0.0, 2.0 * std::f64::consts::PI * d_ns * 1e-9 * freq).exp()
        }
    }
    result
}

pub fn ampl2wgt(ampl: &[f64]) -> Array2<Complex<f64>> {
    assert_eq!(ampl.len(), NPORTS_PER_STATION);
    let mut result = unit_wgt();
    result
        .axis_iter_mut(Axis(1))
        .zip(ampl.iter())
        .for_each(|(mut x, &g)| {
            assert!(g>=0.0 && g<=1.0);
            x.iter_mut().for_each(|x1| *x1 *= g);
        });
    result
}
