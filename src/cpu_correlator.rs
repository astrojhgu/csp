use num_complex::Complex;
use crate::cfg::{RawDataType, NCH_PER_STREAM, NFRAME_PER_PKT, NPKT_PER_CORR};
use rayon::prelude::*;

pub fn correlate(x: &[RawDataType], y: &[RawDataType], corr_result: &mut [f32]) {
    assert_eq!(x.len(), NCH_PER_STREAM * 2 * NFRAME_PER_PKT * NPKT_PER_CORR);
    assert_eq!(x.len(), y.len());
    assert_eq!(corr_result.len(), 2*NCH_PER_STREAM);
    let corr_result1=x.par_chunks(NCH_PER_STREAM*2).zip(y.par_chunks(NCH_PER_STREAM*2)).map(|(x1, y1)|{
        let corr1=x1.chunks(2).zip(y1.chunks(2)).map(|(x11, y11)|{
            let xc=Complex::new(x11[0] as f32, x11[1] as f32);
            let yc=Complex::new(y11[0] as f32, y11[1] as f32);
            yc*xc.conj()
        }).collect::<Vec<_>>();
        corr1
    }).reduce(||{
        vec![Complex::<f32>::default(); NCH_PER_STREAM]
    }, |a,b|{
        a.iter().zip(b.iter()).map(|(a1,b1)|{
            a1+b1
        }).collect()
    });
    corr_result.chunks_mut(2).zip(corr_result1.iter()).for_each(|(a,b)|{
        a[0]=b.re;
        a[1]=b.im;
    });
}
