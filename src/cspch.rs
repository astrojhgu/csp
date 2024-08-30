use crate::{
    bindings::{
        channelize, channelize_no_out, create_channelizer, create_correlator, destroy_channelizer,
        Channelizer,
    },
    cfg::{NCH_PER_STREAM},
};

pub struct CspChannelizer {
    pub nsteps: usize,
    pub nch_coarse: usize,
    pub nch_fine_per_coarse_full: usize,
    c_channelizer: *mut Channelizer,
}

impl CspChannelizer {
    pub fn new(
        nsteps: usize,
        nch_coarse: usize,
        nch_fine_per_coarse_full: usize,
        coeff: &[f32],
    ) -> Self {
        let coeff_ptr = coeff.as_ptr();
        let coeff_len = coeff.len();
        Self {
            nsteps,
            nch_coarse,
            nch_fine_per_coarse_full,
            c_channelizer: unsafe {
                create_channelizer(
                    nsteps,
                    nch_coarse,
                    nch_fine_per_coarse_full,
                    coeff_ptr,
                    coeff_len,
                )
            },
        }
    }

    pub fn channelize(&mut self, input: &[i16], output: &mut [f32]) {
        assert_eq!(input.len(), self.nsteps * self.nch_coarse * 2);
        assert_eq!(output.len(), self.nsteps * self.nch_coarse);
        unsafe { channelize(self.c_channelizer, input.as_ptr(), output.as_mut_ptr()) };
    }

    pub fn channelize_no_out(&mut self, input: &[i16]) {
        assert_eq!(input.len(), self.nsteps * self.nch_coarse * 2);
        unsafe { channelize_no_out(self.c_channelizer, input.as_ptr()) };
    }

    pub fn input_buf_len(&self) -> usize {
        NCH_PER_STREAM * self.nsteps * 2
    }

    pub fn output_buf_len(&self) -> usize {
        NCH_PER_STREAM * self.nsteps
    }
}

impl Drop for CspChannelizer {
    fn drop(&mut self) {
        unsafe { destroy_channelizer(self.c_channelizer) };
    }
}

pub fn calc_coeff(nch: usize, tap_per_ch: usize, k: f32) -> Vec<f32> {
    let mut result = vec![0f32; nch * tap_per_ch];
    unsafe { crate::bindings::calc_coeff(nch, tap_per_ch, k, result.as_mut_ptr()) };
    result
}

pub struct Correlator {
    pub nsteps: usize,
    pub nch: usize,
    pub c_correlator: *mut crate::bindings::Correlator,
}

impl Correlator {
    pub fn new(nch: usize, nsteps: usize) -> Self {
        Self {
            nsteps,
            nch,
            c_correlator: unsafe { create_correlator(nch, nsteps) },
        }
    }

    pub fn correlate(&mut self, ch1: &CspChannelizer, ch2: &CspChannelizer, result: &mut [f32]) {
        assert_eq!(result.len(), self.nch * 2);
        assert_eq!(self.nch * 2, ch1.nch_coarse * ch1.nch_fine_per_coarse_full);
        assert_eq!(self.nch * self.nsteps * 2, ch1.nch_coarse * ch1.nsteps);
        assert_eq!(ch1.nsteps, ch2.nsteps);
        assert_eq!(ch1.nch_coarse, ch2.nch_coarse);
        assert_eq!(ch1.nch_fine_per_coarse_full, ch2.nch_fine_per_coarse_full);
        unsafe {
            crate::bindings::correlate(
                self.c_correlator,
                ch1.c_channelizer,
                ch2.c_channelizer,
                result.as_mut_ptr(),
            )
        }
    }
}
