pub mod utils;
pub mod coffee_ldr;

use std::{fs::{self, File}, io::{Read, BufReader}};
use utils::load_file_into_memory;
use coffee_ldr::coffee_ldr;

fn main() {
    let mut bof_file;    
    let mut entry;
    let mut status: u64;
    let mut size: u64 = 0;
    let mut memory: *mut u8;
    let mut output: i8;
    
    println!("[*] RUST CoffeeLdr: Beacon Object loader by Mark@DUBHE");
    
    //let Entry_arg = std::env::args().nth(1).expect("\n[!] no Entrypoint given\n[*] Help: rustldr [entrypoint] [file]\n");
    //let File_arg  = std::env::args().nth(2).expect("\n[!] no File given\n[*] Help: rustldr [entrypoint] [file]\n");
    let entry_arg = "go";
    let file_arg = "test.bin";
    entry = entry_arg;
    bof_file = file_arg;
    
    println!("[*] File => {}\n", bof_file);

    memory = load_file_into_memory(bof_file, &mut size);  // 加载bof文件
    if memory.is_null() {
        panic!("[!] Couldn't load file")
    }
    println!("{}", size);
    // status = coffee_ldr( entry, memory);

       
}      

