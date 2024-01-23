use std::io::{Read, Write};

pub fn read_data<T: Sized + Default + Clone, R: Read>(source: &mut R, nelements: usize) -> Vec<T> {
    let mut result = vec![T::default(); nelements];
    let buf = unsafe {
        std::slice::from_raw_parts_mut(
            result.as_mut_ptr() as *mut u8,
            result.len() * std::mem::size_of::<T>(),
        )
    };
    source.read_exact(buf).unwrap();
    result
}

pub fn write_data<T: Sized + Default + Clone, W: Write>(drain: &mut W, buf: &[T]) {
    let buf = unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u8,
            buf.len() * std::mem::size_of::<T>(),
        )
    };
    drain.write_all(buf).unwrap();
}
