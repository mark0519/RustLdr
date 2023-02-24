use std::fs::File;
use std::io::{Read};

pub fn load_file_into_memory(path: &str, memory_size: &mut u64) -> *mut u8 {
    let mut file = File::open(path).unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let mut buffer = Vec::with_capacity(file_size);
    unsafe {
        buffer.set_len(file_size);
    }
    file.read_exact(&mut buffer);

    let image_buffer = buffer.as_mut_ptr();
    std::mem::forget(buffer);

    *memory_size = file_size as u64;

    image_buffer
}