extern crate winapi;

use winapi::{
    shared::{
        minwindef::{DWORD, LPVOID, UINT, USHORT, UCHAR},
        ntdef::{CHAR, HANDLE, PVOID, ULONG},
        basetsd::{SIZE_T}
    }
};

pub const COFFAPI_BEACONDATAPARSER:u64             = 0xd0d30e22;
pub const COFFAPI_BEACONDATAINT:u64                = 0xff041492;
pub const COFFAPI_BEACONDATASHORT:u64              = 0xd10d2177;
pub const COFFAPI_BEACONDATALENGTH:u64             = 0xe2262f89;
pub const COFFAPI_BEACONDATAEXTRACT:u64            = 0x38d8c562;


pub const COFFAPI_BEACONFORMATALLOC:u64            = 0x67aab721;
pub const COFFAPI_BEACONFORMATRESET:u64            = 0x68da9d99;
pub const COFFAPI_BEACONFORMATFREE:u64             = 0xf3a32998;
pub const COFFAPI_BEACONFORMATAPPEND:u64           = 0x5d4c05ee;
pub const COFFAPI_BEACONFORMATPRINTF:u64           = 0x8069e8c9;
pub const COFFAPI_BEACONFORMATTOSTRING:u64         = 0x245f03f0;
pub const COFFAPI_BEACONFORMATINT:u64              = 0x2669d741;


pub const COFFAPI_BEACONPRINTF:u64                 = 0x89bf3d20;
pub const COFFAPI_BEACONOUTPUT:u64                 = 0x87a66ede;
pub const COFFAPI_BEACONUSETOKEN:u64               = 0xd7dbbb5b;
pub const COFFAPI_BEACONREVERTTOKEN:u64            = 0xd7421e6;
pub const COFFAPI_BEACONISADMIN:u64                = 0xa88e0392;
pub const COFFAPI_BEACONGETSPAWNTO:u64             = 0x32e13a39;
pub const COFFAPI_BEACONSPAWNTEMPORARYPROCESS:u64  = 0xad80158;
pub const COFFAPI_BEACONINJECTPROCESS:u64          = 0xe8f5bd09;
pub const COFFAPI_BEACONINJECTTEMPORARYPROCESS:u64 = 0x96fbf28c;
pub const COFFAPI_BEACONCLEANUPPROCESS:u64         = 0xa0dc954;


pub const COFFAPI_TOWIDECHAR:u64                   = 0x5cec66cf;
pub const COFFAPI_LOADLIBRARYA:u64                 = 0xb7072fdb;
pub const COFFAPI_GETPROCADDRESS:u64               = 0xdecfc1bf;
pub const COFFAPI_GETMODULEHANDLE:u64              = 0xd908e1d8;
pub const COFFAPI_FREELIBRARY:u64                  = 0x4ad9b11c;

struct COFFAPIFUNC {
    name_hash: *mut u64,
    pointer: *mut i8,
}