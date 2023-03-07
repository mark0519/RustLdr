use std::{ffi::{c_void, CString, OsStr, CStr}, fs::File, io::Read, mem, os::windows::ffi::OsStrExt, ptr::null_mut};
use windows::{
    core::*, Win32::Foundation::*, Win32::System::*,
};
use std::mem::size_of;
use std::alloc::{alloc_zeroed, Layout};
use crate::utils::hash_string;
use crate::beacon_api::*;
const COFF_PREP_SYMBOL: u32 = 0xec598a48;
const COFF_PREP_SYMBOL_SIZE: u32 = 6;
const COFF_PREP_BEACON: u32 = 0x353400b0;
const COFF_PREP_BEACON_SIZE: u32 = COFF_PREP_SYMBOL_SIZE + 6;

const IMAGE_REL_AMD64_ADDR64: u16 = 0x0001;
const IMAGE_REL_AMD64_ADDR32NB: u16 = 0x0003;
const IMAGE_REL_AMD64_REL32: u16 = 0x0004;
const IMAGE_REL_AMD64_REL32_5: u16 = 0x0009;

const BeaconApiCounter:u32 = 25;

#[repr(C)]
struct COFF_FILE_HEADER { // 文件头
    Machine: u32,
    NumberOfSections: u32,
    TimeDateStamp: u32,
    PointerToSymbolTable: u32,
    NumberOfSymbols: u32,
    SizeOfOptionalHeader: u32,
    Characteristics: u32,
}

#[repr(C)]
struct COFF_SECTION { // 节区表
    Name: [CHAR; 8],
    VirtualSize: u32,
    VirtualAddress: u32,
    SizeOfRawData: u32,
    PointerToRawData: u32,
    PointerToRelocations: u32,
    PointerToLineNumbers: u32,
    NumberOfRelocations: u32,
    NumberOfLinenumbers: u32,
    Characteristics: u32,
}

#[repr(C)]
struct COFF_RELOC { //重定位信息表
    VirtualAddress: u32,
    SymbolTableIndex: u32,
    Type: u16,
}

#[repr(C)]
union COFFSymbolFirst {
    Name: [CHAR; 8],
    Value: [u32; 2]
}

#[repr(C)]
struct COFF_SYMBOL { //符号表
    First: COFFSymbolFirst,
    SectionNumber: u16,
    Type: u16,
    StorageClass: u8,
    NumberOfAuxSymbols: u8,
}

#[repr(C)]
struct SECTION_MAP { //节区映射表
    ptr: *mut i8,
    size: usize,
}

#[repr(C)]
struct COFFEE {
    data: *mut c_void,
    header: *mut COFF_FILE_HEADER,
    section: *mut COFF_SECTION,
    reloc: *mut COFF_RELOC,
    symbol: *mut COFF_SYMBOL,
    sec_map: *mut SECTION_MAP,
    fun_map: *mut i8,
}

impl std::fmt::Debug for COFFEE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COFFEE")
            .field("data", &self.data)
            .field("header", &self.header)
            .field("section", &self.section)
            .field("reloc", &self.reloc)
            .field("symbol", &self.symbol)
            .field("sec_map", &self.sec_map)
            .field("fun_map", &self.fun_map)
            .finish()
    }
}


unsafe fn coffee_process_symbol(symbol: *mut i8) -> *mut i8   {
    let mut bak: [u8; 1024] = [0; 1024];
    let mut func_addr: *mut i8 = null_mut();
    let mut sym_library: *mut i8 = null_mut();
    let mut sym_function: *mut i8 = null_mut();
    // let mut h_library: HANDLE = null_mut();

    let mut sym_hash = hash_string((symbol as u32 + COFF_PREP_SYMBOL_SIZE) as *mut i8, 0);
    let mut sym_beacon = hash_string(symbol, COFF_PREP_BEACON_SIZE as usize);

    std::ptr::copy_nonoverlapping(
        symbol,
        bak.as_mut_ptr() as *mut i8,
        CStr::from_ptr(symbol).to_str().unwrap().len() + 1 as usize
    );

    if sym_beacon == COFF_PREP_BEACON       ||  // check if this is a Beacon api
        sym_hash == COFFAPI_TOWIDECHAR      ||
        sym_hash == COFFAPI_GETPROCADDRESS  ||
        sym_hash == COFFAPI_LOADLIBRARYA    ||
        sym_hash == COFFAPI_GETMODULEHANDLE ||
        sym_hash == COFFAPI_FREELIBRARY
    {
        sym_function = symbol.offset(COFF_PREP_SYMBOL_SIZE as isize);
        for i in 0.. BeaconApiCounter{
            // if(hash_string(sym_function, 0) == BeaconApi[i].NameHash)
            // {
            //     ToDo...
            //     BeaconApi[i].Pointer;
            // }
        }
    }


    func_addr // TODO: check
}


pub fn coffee_ldr(entry_name: &str, coffee_data: *const c_void, arg_data: *const c_void, arg_size: usize) -> u32 {
    let mut coffee = COFFEE {
        data: null_mut(),
        header: null_mut(),
        section: null_mut(),
        reloc: null_mut(),
        symbol: null_mut(),
        sec_map: null_mut(),
        fun_map: null_mut(),
    };

    if coffee_data.is_null() {
        println!("[!] Coffee data is empty");
        return 1;
    }

    coffee.data = coffee_data as *mut c_void;
    coffee.header = coffee_data as *mut COFF_FILE_HEADER;

    unsafe {
        let sec_map_size = (coffee.header as *const _ as usize).wrapping_add(size_of::<COFF_SECTION>() * (*coffee.header).NumberOfSections as usize);
        let mut sec_vec = Vec::<SECTION_MAP>::with_capacity(sec_map_size / size_of::<SECTION_MAP>());
        coffee.sec_map = sec_vec.as_mut_ptr();
        // 通过调用 std::mem::forget() 来确保内存不会被释放
        std::mem::forget(sec_vec);

        // 创建一个大小为 2048 字节的 Layout 对象
        let layout = Layout::from_size_align(2048, std::mem::align_of::<i8>()).unwrap();
        // 分配一个未初始化的内存块
        let ptr = alloc_zeroed(layout) ;
        // 将指针转换为一个 Vec<i8> 类型的引用
        let mut fun_vec = Vec::from_raw_parts(ptr as *mut i8, layout.size(), layout.size()) ;
        coffee.fun_map = fun_vec.as_mut_ptr();
        // 通过调用 std::mem::forget() 来确保内存不会被释放
        std::mem::forget(fun_vec);

        println!("[*] Load sections");

        for i in 0..(*coffee.header).NumberOfSections {
            coffee.section = ((coffee_data as usize) + size_of::<COFF_FILE_HEADER>() + size_of::<COFF_SECTION>()*(i as usize) ) as *mut COFF_SECTION;
            (*coffee.sec_map.offset(i as isize)).size = (*coffee.section).SizeOfRawData as usize;
            let mut vec = Vec::<u8>::with_capacity((*coffee.sec_map.offset(i as isize)).size);
            let ptr = vec.as_mut_ptr() as *mut i8;
            (*coffee.sec_map.offset(i as isize)).ptr = ptr;
            // 通过调用 std::mem::forget() 来确保内存不会被释放
            std::mem::forget(vec);
            let src = coffee_data as *const u8;
            let dest = (*coffee.sec_map.offset(i as isize)).ptr as *mut u8;
            std::ptr::copy_nonoverlapping(src.offset((*coffee.section).PointerToRawData as isize), dest, (*coffee.section).SizeOfRawData as usize);
        }
    
        println!("[*] Process sections");
        coffee.symbol = (coffee_data as *mut COFF_SECTION).offset(1) as *mut COFF_SYMBOL;

        println!("[*] Debug: {:?}", coffee);  // for debug
        if !coffee_process_sections(&mut coffee) {
            println!("[*] Failed to process relocation");
            return 1;
        }
    }

    println!("[*] Execute coffee main");
    // coffee_execute_function(&coffee, entry_name, arg_data, arg_size);

    println!("[*] Cleanup memory");
    // coffee_cleanup(&mut coffee);

    0
}

fn coffee_process_sections(coffee: &mut COFFEE) -> bool {
    let mut symbol:u32 = 0;
    let mut sym_string:*mut i8;
    let mut func_ptr:*mut i8;
    let mut func_count = 0;
    let mut offset_long:u64 = 0;
    let mut offset:u32 = 0;
    unsafe{
        for section_cnt in 0..(*coffee.header).NumberOfSections{
            coffee.section = ((coffee.data as usize) + size_of::<COFF_FILE_HEADER>() + size_of::<COFF_SECTION>()*(section_cnt as usize) ) as *mut COFF_SECTION;
            coffee.reloc = ((coffee.data as usize) + (*coffee.section).PointerToRelocations as usize ) as *mut COFF_RELOC;

            for reloc_cnt in 0..(*coffee.section).NumberOfRelocations{
                if (*coffee.symbol.offset((*coffee.reloc).SymbolTableIndex as isize)).First.Name[0] != windows::Win32::Foundation::CHAR(0) {
                    symbol = (*coffee.symbol.offset((*coffee.reloc).SymbolTableIndex as isize)).First.Value[1];
                    
                    if (*coffee.reloc).Type == IMAGE_REL_AMD64_ADDR64 {
                        std::ptr::copy_nonoverlapping(
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as usize + (*coffee.reloc).VirtualAddress as usize) as *mut u64,
                            &mut offset_long as *mut u64,
                            std::mem::size_of::<u64>(),
                        );
                        let symbol_index:u32 = (*coffee.reloc).SymbolTableIndex;
                        let sec_map_index:u32 = ((*coffee.symbol.offset(symbol_index as isize)).SectionNumber - 1).into();
                        offset_long = (*coffee.sec_map.offset( sec_map_index as isize )).ptr as u64 + offset_long;
                        std::ptr::copy_nonoverlapping(
                            &mut offset_long as *mut u64,
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as usize + (*coffee.reloc).VirtualAddress as usize) as *mut u64,
                            std::mem::size_of::<u64>(),
                        );
                    } else if ((*coffee.reloc).Type == IMAGE_REL_AMD64_ADDR32NB) {
                        std::ptr::copy_nonoverlapping(
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as usize + (*coffee.reloc).VirtualAddress as usize) as *mut u32,
                            &mut offset as *mut u32,
                            std::mem::size_of::<u32>(),
                        );
                        let symbol_index:u32 = (*coffee.reloc).SymbolTableIndex;
                        let sec_map_index:u32 = ((*coffee.symbol.offset(symbol_index as isize)).SectionNumber - 1).into();
                        offset = (*coffee.sec_map.offset( sec_map_index as isize )).ptr as u32 + offset;
                        if offset as u64 - (((*coffee.sec_map.offset(section_cnt as isize)).ptr as u64 + (*coffee.reloc).VirtualAddress as u64 + 4) as u64)  > 0xffffffff {
                            return false;
                        }
                        offset = offset - (((*coffee.sec_map.offset(section_cnt as isize)).ptr as u32 + (*coffee.reloc).VirtualAddress as u32 + 4) as u32);
                        std::ptr::copy_nonoverlapping(
                            &mut offset as *mut u32,
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as usize + (*coffee.reloc).VirtualAddress as usize) as *mut u32,
                            std::mem::size_of::<u32>(),
                        );
                    } else if (IMAGE_REL_AMD64_REL32 <= (*coffee.reloc).Type &&
                              (*coffee.reloc).Type <= IMAGE_REL_AMD64_REL32_5) {
                        std::ptr::copy_nonoverlapping(
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as usize + (*coffee.reloc).VirtualAddress as usize) as *mut u32,
                            &mut offset as *mut u32,
                            std::mem::size_of::<u32>(),
                        );
                        let symbol_index:u32 = (*coffee.reloc).SymbolTableIndex;
                        let sec_map_index:u32 = ((*coffee.symbol.offset(symbol_index as isize)).SectionNumber - 1).into();
                        let tmp_ptr = (*coffee.sec_map.offset(sec_map_index as isize)).ptr;
                        if((tmp_ptr as u64 - (*coffee.reloc).VirtualAddress as u64 + 4) as u64 > 0xffffffff) {
                            return false;
                        }
                        offset += tmp_ptr as u32 - ((*coffee.reloc).Type - 4 ) as u32 - ((*coffee.sec_map.offset(section_cnt as isize)).ptr as u32 + (*coffee.reloc).VirtualAddress + 4) as u32;
                        std::ptr::copy_nonoverlapping(
                            &mut offset as *mut u32,
                            ((*coffee.sec_map.offset(section_cnt as isize)).ptr as u32 + (*coffee.reloc).VirtualAddress as u32) as *mut u32,
                            std::mem::size_of::<u32>(),
                        );
                    }else{
                        println!("[!] Relocation type not found: {}",(*coffee.reloc).Type);
                    }
                }else{
                    symbol = (*coffee.symbol.offset((*coffee.reloc).SymbolTableIndex as isize)).First.Value[1];
                    sym_string = ((coffee.symbol as usize + (*coffee.header).NumberOfSymbols as usize) + symbol as usize) as *mut i8;
                    func_ptr = coffee_process_symbol( sym_string );
                    // TODO: need finish coffee_process_symbol
                
                }

            }

        }
    }




    true
}


