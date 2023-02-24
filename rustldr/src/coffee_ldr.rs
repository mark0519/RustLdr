use std::{fs::{self, File}, io::{Read}};
use std::mem::size_of;

struct CoffFileHeader {  //COFF 文件头
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

struct CoffSection {  //COFF 节区表
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_linenumbers: u16,
    characteristics: u32,
}

struct CoffReloc {  // 重定位信息表
    virtual_address: u32,
    symbol_table_index: u32,
    r#type: u16,
}

struct CoffSymbol {  // 符号表
    first: [u8; 8],
    value: u32,
    section_number: u16,
    r#type: u16,
    storage_class: u8,
    number_of_aux_symbols: u8,
}

struct SectionMap {  //节区映射
    ptr: *mut u8,
    size: usize,
}

struct CoffEE { // COFF 加载器
    data: Box<[u8]>,
    header: *mut CoffFileHeader,
    section: *mut CoffSection,
    reloc: *mut CoffReloc,
    symbol: *mut CoffSymbol,
    sec_map: *mut SectionMap,
    fun_map: *mut u8,
}



pub fn coffee_ldr(entry_name: &str, coffee_data:Box<[u8]>) -> u64 {
    let mut coffee:CoffEE;
    // coffee.data = coffee_data;
    // coffee.header = coffee.data.clone();

    1
}
