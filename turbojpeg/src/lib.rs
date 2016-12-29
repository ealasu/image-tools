pub mod bindings;

const COLOR_COMPONENTS: usize = 3;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn decompress(compressed_image: &[u8]) -> Image {
    let decompressor = unsafe {
        bindings::tjInitDecompress()
    };

    let mut subsamp = 0;
    let mut width = 0;
    let mut height = 0;
    let mut colorspace = 0;
    
    unsafe {
        bindings::tjDecompressHeader3(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            &mut width,
            &mut height,
            &mut subsamp,
            &mut colorspace
        );
    }

    let buf_len: usize = width as usize * height as usize * COLOR_COMPONENTS;
    let mut buffer: Vec<u8> = Vec::with_capacity(buf_len);
    unsafe { buffer.set_len(buf_len); }

    unsafe {
        bindings::tjDecompress2(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            buffer.as_mut_ptr(),
            width,
            0, // pitch
            height,
            bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
            bindings::TJFLAG_FASTDCT as ::std::os::raw::c_int);
    }
    unsafe {
        bindings::tjDestroy(decompressor);
    }

    Image {
        width: width as u32,
        height: height as u32,
        data: buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn test() {
        let mut jpeg = Vec::new();
        File::open("test/in.jpg").unwrap().read_to_end(&mut jpeg).unwrap();
        let image = decompress(&jpeg);
        println!("width: {}, height: {}", image.width, image.height);

        let mut expected_raw = Vec::new();
        File::open("test/out.dat").unwrap().read_to_end(&mut expected_raw).unwrap();
        assert_eq!(image.data, expected_raw);
    }
}
