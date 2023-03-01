use std::fs::File;
use std::io::{Read};
const HASH_KEY: u32 = 5381;

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

pub unsafe fn hash_string(string: *mut i8, length: usize) -> u32 {
    let mut hash: u32 = HASH_KEY;
    let mut ptr: *mut i8 = string;

    for i in 0..length {
        let c = *ptr.offset(i as isize);
        if c == 0 {
            break;
        }
        let character = if c >= 97 { c - 32 } else { c };
        hash = hash.wrapping_mul(33).wrapping_add(character as u32);
    }
    

    hash
}