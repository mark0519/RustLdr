#![feature(c_variadic)]

use std::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub const COFFAPI_BEACONDATAPARSER:u32             = 0xd0d30e22;
pub const COFFAPI_BEACONDATAINT:u32                = 0xff041492;
pub const COFFAPI_BEACONDATASHORT:u32              = 0xd10d2177;
pub const COFFAPI_BEACONDATALENGTH:u32             = 0xe2262f89;
pub const COFFAPI_BEACONDATAEXTRACT:u32            = 0x38d8c562;


pub const COFFAPI_BEACONFORMATALLOC:u32            = 0x67aab721;
pub const COFFAPI_BEACONFORMATRESET:u32            = 0x68da9d99;
pub const COFFAPI_BEACONFORMATFREE:u32             = 0xf3a32998;
pub const COFFAPI_BEACONFORMATAPPEND:u32           = 0x5d4c05ee;
pub const COFFAPI_BEACONFORMATPRINTF:u32           = 0x8069e8c9;
pub const COFFAPI_BEACONFORMATTOSTRING:u32         = 0x245f03f0;
pub const COFFAPI_BEACONFORMATINT:u32              = 0x2669d741;


pub const COFFAPI_BEACONPRINTF:u32                 = 0x89bf3d20;
pub const COFFAPI_BEACONOUTPUT:u32                 = 0x87a66ede;
pub const COFFAPI_BEACONUSETOKEN:u32               = 0xd7dbbb5b;
pub const COFFAPI_BEACONREVERTTOKEN:u32            = 0xd7421e6;
pub const COFFAPI_BEACONISADMIN:u32                = 0xa88e0392;
pub const COFFAPI_BEACONGETSPAWNTO:u32             = 0x32e13a39;
pub const COFFAPI_BEACONSPAWNTEMPORARYPROCESS:u32  = 0xad80158;
pub const COFFAPI_BEACONINJECTPROCESS:u32          = 0xe8f5bd09;
pub const COFFAPI_BEACONINJECTTEMPORARYPROCESS:u32 = 0x96fbf28c;
pub const COFFAPI_BEACONCLEANUPPROCESS:u32         = 0xa0dc954;


pub const COFFAPI_TOWIDECHAR:u32                   = 0x5cec66cf;
pub const COFFAPI_LOADLIBRARYA:u32                 = 0xb7072fdb;
pub const COFFAPI_GETPROCADDRESS:u32               = 0xdecfc1bf;
pub const COFFAPI_GETMODULEHANDLE:u32              = 0xd908e1d8;
pub const COFFAPI_FREELIBRARY:u32                  = 0x4ad9b11c;

struct Datap {
    original: *mut i8,
    buffer: *mut i8,
    length: i32,
    size: i32,
}

struct Formatp {
    original: *mut i8,
    buffer: *mut i8,
    length: i32,
    size: i32,
}

fn BeaconDataParse(parser: *mut Datap, buffer: *mut i8, size: i32){
// implementation
}
fn BeaconDataInt(parser: *mut Datap) -> i32{
// implementation
}
fn BeaconDataShort(parser: *mut Datap) -> i16{
// implementation
}
fn BeaconDataLength(parser: *mut Datap) -> i32{
// implementation
}
fn BeaconDataExtract(parser: *mut Datap, size: *mut i32) -> *mut i8{
// implementation
}

fn BeaconFormatAlloc(format: *mut Formatp, maxsz: i32){
// implementation
}
fn BeaconFormatReset(format: *mut Formatp){
// implementation
}
fn BeaconFormatFree(format: *mut Formatp){
// implementation
}
fn BeaconFormatAppend(format: *mut Formatp, text: *mut i8, len: i32){
// implementation
}
fn BeaconFormatPrintf(format: *mut Formatp, fmt: *const c_char, ...) -> c_void{
        // implementation
}
fn BeaconFormatToString(format: *mut Formatp, size: *mut i32) -> *mut i8{
// implementation
}
fn BeaconFormatInt(format: *mut Formatp, value: i32){
// implementation
}

const CALLBACK_OUTPUT: i32 = 0x0;
const CALLBACK_OUTPUT_OEM: i32 = 0x1e;
const CALLBACK_ERROR: i32 = 0x0d;
const CALLBACK_OUTPUT_UTF8: i32 = 0x20;

fn BeaconPrintf(t: i32, fmt: &str, ...) {
// implementation
}

fn BeaconOutput(t: i32, data: &str, len: i32) {
// implementation
}

/* Token Functions */
fn BeaconUseToken(token: HANDLE) -> bool {
// implementation
}

fn BeaconRevertToken() {
// implementation
}

fn BeaconIsAdmin() -> bool {
// implementation
}

/* Spawn+Inject Functions */
fn BeaconGetSpawnTo(x86: bool, buffer: &mut [char], length: i32) {
// implementation
}

fn BeaconSpawnTemporaryProcess(x86: bool, ignoreToken: bool, sInfo: &STARTUPINFO, pInfo: &PROCESS_INFORMATION) -> bool {
// implementation
}

fn BeaconInjectProcess(hProc: HANDLE, pid: i32, payload: &str, p_len: i32, p_offset: i32, arg: &str, a_len: i32) {
// implementation
}

fn BeaconInjectTemporaryProcess(pInfo: &PROCESS_INFORMATION, payload: &str, p_len: i32, p_offset: i32, arg: &str, a_len: i32) {
// implementation
}

fn BeaconCleanupProcess(pInfo: &PROCESS_INFORMATION) {
// implementation
}

/* Utility Functions */
fn toWideChar(src: &str, dst: &mut [wchar_t], max: i32) -> bool {
// implementation
}

fn swap_endianess(indata: u32) -> u32 {
// implementation
}

fn BeaconGetOutputData(outsize: &mut i32) -> &str {
// implementation
}


type COFFAPIFUNC = (u32, *mut std::ffi::c_void);

static BEACON_API: [COFFAPIFUNC; 22] = [
    (COFFAPI_BEACONDATAPARSER,             BeaconDataParse              as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAINT,                BeaconDataInt                as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATASHORT,              BeaconDataShort              as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATALENGTH,             BeaconDataLength             as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAEXTRACT,            BeaconDataExtract            as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATALLOC,            BeaconFormatAlloc            as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATRESET,            BeaconFormatReset            as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATFREE,             BeaconFormatFree             as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATAPPEND,           BeaconFormatAppend           as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATPRINTF,           BeaconFormatPrintf           as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATTOSTRING,         BeaconFormatToString         as *mut std::ffi::c_void),
    (COFFAPI_BEACONFORMATINT,              BeaconFormatInt              as *mut std::ffi::c_void),
    (COFFAPI_BEACONPRINTF,                 BeaconPrintf                 as *mut std::ffi::c_void),
    (COFFAPI_BEACONOUTPUT,                 BeaconOutput                 as *mut std::ffi::c_void),
    (COFFAPI_BEACONUSETOKEN,               BeaconUseToken               as *mut std::ffi::c_void),
    (COFFAPI_BEACONREVERTTOKEN,            BeaconRevertToken            as *mut std::ffi::c_void),
    (COFFAPI_BEACONISADMIN,                BeaconIsAdmin                as *mut std::ffi::c_void),
    (COFFAPI_BEACONGETSPAWNTO,             BeaconGetSpawnTo             as *mut std::ffi::c_void),
    (COFFAPI_BEACONINJECTPROCESS,          BeaconInjectProcess          as *mut std::ffi::c_void),
    (COFFAPI_BEACONINJECTTEMPORARYPROCESS, BeaconInjectTemporaryProcess as *mut std::ffi::c_void),
    (COFFAPI_BEACONCLEANUPPROCESS,         BeaconCleanupProcess         as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAPARSER,             toWideChar                   as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAPARSER,             LoadLibraryA                 as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAPARSER,             GetProcAddress               as *mut std::ffi::c_void),
    (COFFAPI_BEACONDATAPARSER,             FreeLibrary                  as *mut std::ffi::c_void),
];