use std::{fs::{self, File}, io::{Read}};

pub fn load_file_into_memory(path:&str, mut memory_size:u64) -> Box<[u8]> {
    let len:u64 = fs::metadata(path).unwrap().len();
    memory_size = len; // 获得bof文件实际大小
    
    let mut h_file:File = fs::File::open(path).expect("\n[!] Error opening bof file\n");
    let mut buf:Vec<u8> = Vec::with_capacity(memory_size as usize); // 根据bof文件实际大小开辟缓冲区
    h_file.read_to_end(&mut buf);
    buf.into_boxed_slice()
}