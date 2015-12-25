use std::mem;


pub fn convert_vec<From,To>(mut data: Vec<From>) -> Vec<To> {
    data.shrink_to_fit();
    let p = data.as_mut_ptr();
    let len = data.len() * mem::size_of::<From>() / mem::size_of::<To>();
    unsafe {
        mem::forget(data);
        Vec::from_raw_parts(p as *mut To, len, len)
    }
}
