extern crate winapi;

use std::{fs::{self, File}, io::{Read}, mem, default};
use std::mem::{size_of, zeroed};
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::os::windows::ffi::OsStrExt;
use std::os::raw::c_void;
use winapi::{um::{memoryapi::{VirtualAlloc, VirtualProtect}}, shared::basetsd::UINT64};
use winapi::{
    shared::{
        minwindef::{DWORD, LPVOID, UINT, USHORT, UCHAR},
        ntdef::{CHAR, HANDLE, PVOID, ULONG},
        basetsd::{SIZE_T}
    },
    um::{
        winnt::{
            MEM_COMMIT, MEM_RESERVE, MEM_TOP_DOWN, PAGE_EXECUTE_READWRITE, PAGE_READWRITE,
            PIMAGE_BASE_RELOCATION, PIMAGE_IMPORT_DESCRIPTOR, PIMAGE_NT_HEADERS,
            PIMAGE_SECTION_HEADER, PIMAGE_THUNK_DATA, PIMAGE_TLS_CALLBACK,
        },
    winnt::{PCHAR, IMAGE_REL_AMD64_ADDR64, IMAGE_REL_AMD64_ADDR32NB, IMAGE_REL_AMD64_REL32, IMAGE_REL_AMD64_REL32_5}
    },
};

const COFF_PREP_SYMBOL: u64 = 0xec598a48;
const COFF_PREP_SYMBOL_SIZE: u64 = 6;
const COFF_PREP_BEACON: u64 = 0x353400b0;
const COFF_PREP_BEACON_SIZE: u64 = COFF_PREP_SYMBOL_SIZE + 6;


#[repr(C)]
struct COFF_FILE_HEADER { // 文件头
    Machine: UINT,
    NumberOfSections: UINT,
    TimeDateStamp: ULONG,
    PointerToSymbolTable: ULONG,
    NumberOfSymbols: ULONG,
    SizeOfOptionalHeader: UINT,
    Characteristics: UINT,
}

#[repr(C)]
struct COFF_SECTION { // 节区表
    Name: [CHAR; 8],
    VirtualSize: ULONG,
    VirtualAddress: ULONG,
    SizeOfRawData: ULONG,
    PointerToRawData: ULONG,
    PointerToRelocations: ULONG,
    PointerToLineNumbers: ULONG,
    NumberOfRelocations: UINT,
    NumberOfLinenumbers: UINT,
    Characteristics: UINT,
}

#[repr(C)]
struct COFF_RELOC { //重定位信息表
    VirtualAddress: ULONG,
    SymbolTableIndex: ULONG,
    Type: USHORT,
}

#[repr(C)]
union COFFSymbolFirst {
    Name: [CHAR; 8],
    Value: [ULONG; 2]
}

#[repr(C)]
struct COFF_SYMBOL { //符号表
    First: COFFSymbolFirst,
    SectionNumber: USHORT,
    Type: USHORT,
    StorageClass: UCHAR,
    NumberOfAuxSymbols: UCHAR,
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

pub fn coffee_ldr(entry_name: &str, coffee_data: *const c_void, arg_data: *const c_void, arg_size: SIZE_T) -> DWORD {
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
        coffee.sec_map = VirtualAlloc(null_mut(), (coffee.header as *const _ as usize).wrapping_add(size_of::<COFF_SECTION>() * (*coffee.header).NumberOfSections as usize) as SIZE_T, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) as *mut SECTION_MAP;

        coffee.fun_map = VirtualAlloc(null_mut(), 2048, MEM_COMMIT | MEM_RESERVE | MEM_TOP_DOWN, PAGE_READWRITE) as *mut i8;

        println!("[*] Load sections");

        for i in 0..(*coffee.header).NumberOfSections {
            coffee.section = ((coffee_data as usize) + size_of::<COFF_FILE_HEADER>() + size_of::<COFF_SECTION>()*(i as usize) ) as *mut COFF_SECTION;
            // coffee.section = coffee_data as *mut COFF_SECTION;
            // coffee.section = coffee.section.offset(1) as *mut COFF_SECTION;
            // coffee.section = coffee.section.offset(i as isize);
            (*coffee.sec_map.offset(i as isize)).size = (*coffee.section).SizeOfRawData as SIZE_T;
            (*coffee.sec_map.offset(i as isize)).ptr = VirtualAlloc(null_mut(), (*coffee.sec_map.offset(i as isize)).size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) as *mut i8;

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
    let mut func_count:DWORD = 0;
    let mut offset_long:u64 = 0;
    let mut offset:u32 = 0;
    unsafe{
        for section_cnt in 0..(*coffee.header).NumberOfSections{
            coffee.section = ((coffee.data as usize) + size_of::<COFF_FILE_HEADER>() + size_of::<COFF_SECTION>()*(section_cnt as usize) ) as *mut COFF_SECTION;
            coffee.reloc = ((coffee.data as usize) + (*coffee.section).PointerToRelocations as usize ) as *mut COFF_RELOC;

            for reloc_cnt in 0..(*coffee.section).NumberOfRelocations{
                if((*coffee.symbol.offset((*coffee.reloc).SymbolTableIndex as isize)).First.Name[0] != 0){
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
                    // func_ptr = coffee_process_symbol( SymString );
                    // TODO: need finish coffee_process_symbol
                
                }

            }

        }
    }




    true
}
