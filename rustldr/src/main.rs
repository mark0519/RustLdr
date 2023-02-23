use std::{fs::{self, File}, io::{Read, BufReader}};

fn load_file_into_memory(path:&str, memory_size:u64)  {
    //DWORD   dwBytesRead = 0;
    match memory_size{
        0 => {
            panic!("\n[!] Error memory_size: {}\n",memory_size);
        },
        _ => {
            let len:u64 = fs::metadata(path).unwrap().len();
            memory_size = len; // 获得bof文件实际大小
        }
    }
    let h_file:File = fs::File::open(path).expect("\n[!] Error opening bof file\n");
    let mut reader:BufReader<File> = BufReader::with_capacity(memory_size as usize, h_file); // 根据bof文件实际大小开辟缓冲区
    
    //TODO:Box<T>
    
    

    
}

fn main() {
    let mut bof_file;    
    let mut entry;
    let mut status: u64;
    let mut size: u64;
    let mut memory: std::os::raw::c_void;
    let mut output: i8;
    
    println!("[*] RUST CoffeeLdr: Beacon Object loader by Mark@DUBHE");
    
    //let Entry_arg = std::env::args().nth(1).expect("\n[!] no Entrypoint given\n[*] Help: Ldr [entrypoint] [file]\n");
    //let File_arg  = std::env::args().nth(2).expect("\n[!] no File given\n[*] Help: Ldr [entrypoint] [file]\n");
    let entry_arg = "go";
    let file_arg = "test.o";
    entry = entry_arg;
    bof_file = file_arg;
    
    println!("[*] File => {}\n", bof_file);
    memory = load_file_into_memory(bof_file, &mut size);
    
       
}

