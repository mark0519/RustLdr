pub mod utils;
pub mod coffee_ldr;

use std::{fs::{self, File}, io::{Read, BufReader}};
use utils::{load_file_into_memory, hash_string};
use coffee_ldr::coffee_ldr;
use std::ptr::null_mut;
use std::os::raw::c_void;

fn main() {
    let mut bof_file;    
    let mut entry;
    let mut status: u32;
    let mut size: u64 = 0;
    let mut memory: *mut u8;
    let mut output: i8;
    
    println!("[*] RUST CoffeeLdr: Beacon Object loader by Mark@DUBHE");
    
    //let entry_arg = std::env::args().nth(1).expect("\n[!] no Entrypoint given\n[*] Help: rustldr [entrypoint] [file]\n");
    //let file_arg  = std::env::args().nth(2).expect("\n[!] no File given\n[*] Help: rustldr [entrypoint] [file]\n");
    let entry_arg = "go";
    let file_arg = "../../../../obf/dome.o";
    entry = entry_arg;
    bof_file = file_arg;
    
    println!("[*] File => {}\n", bof_file);

    memory = load_file_into_memory(bof_file, &mut size);  // 加载bof文件
    if memory.is_null() {
        panic!("[!] Couldn't load file")
    }
    println!("bof_file size: {}", size); // for debug
    status = coffee_ldr( entry, memory as *const c_void, null_mut(), 0)
       
}      



