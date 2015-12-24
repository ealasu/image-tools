use std::mem;
use std::f32;
use std::u16;


pub struct Wrap<T>(pub T);

impl From<Vec<f32>> for Wrap<Vec<u16>> {
    fn from(data: Vec<f32>) -> Self {
        // rescale
        let src_min = data.iter().fold(f32::MAX, |acc, &v| acc.min(v));
        let src_max = data.iter().fold(f32::MIN, |acc, &v| acc.max(v));
        let src_d = src_max - src_min;
        let dst_min = u16::MIN as f32;
        let dst_max = u16::MAX as f32;
        let dst_d = dst_max - dst_min;

        let mut out: Vec<u16> = Vec::with_capacity(data.len());
        for v in data.iter() {
            out.push((((*v - src_min) * dst_d) / src_d) as u16);
        }
        Wrap(out)
    }
}

macro_rules! impl_from_to {
    ($from:ty, $to:ty) => {
        impl From<Vec<$from>> for Wrap<Vec<$to>> {
            fn from(mut data: Vec<$from>) -> Self {
                data.shrink_to_fit();
                let p = data.as_mut_ptr();
                let len = data.len() / mem::size_of::<$to>();
                unsafe {
                    mem::forget(data);
                    Wrap(Vec::from_raw_parts(p as *mut $to, len, len))
                }
            }
        }
    };
}

impl_from_to!(u8, f32); 
impl_from_to!(u8, u16); 
impl_from_to!(f32, u8); 
impl_from_to!(u16, u8); 

//impl<T> From<Vec<T>> for Wrap<Vec<u8>> {
    //fn from(mut data: Vec<T>) -> Self {
        //data.shrink_to_fit();
        //let p = data.as_mut_ptr();
        //let len = data.len() * mem::size_of::<T>();
        //unsafe {
            //mem::forget(data);
            //Wrap(Vec::from_raw_parts(p as *mut u8, len, len))
        //}
    //}
//}
