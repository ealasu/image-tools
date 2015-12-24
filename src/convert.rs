use std::mem;


pub struct Wrap<T>(pub T);

macro_rules! impl_from_to {
    ($from:ty, $to:ty) => {
        impl From<Vec<$from>> for Wrap<Vec<$to>> {
            fn from(mut data: Vec<$from>) -> Self {
                data.shrink_to_fit();
                let p = data.as_mut_ptr();
                let len = data.len() * mem::size_of::<$from>() / mem::size_of::<$to>();
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
