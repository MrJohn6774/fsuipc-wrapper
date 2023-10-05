/* automatically generated by rust-bindgen 0.68.1 */

pub const WINAPI_FAMILY_PC_APP: u32 = 2;
pub const WINAPI_FAMILY_PHONE_APP: u32 = 3;
pub const WINAPI_FAMILY_SYSTEM: u32 = 4;
pub const WINAPI_FAMILY_SERVER: u32 = 5;
pub const WINAPI_FAMILY_GAMES: u32 = 6;
pub const WINAPI_FAMILY_DESKTOP_APP: u32 = 100;
pub const WINAPI_FAMILY_APP: u32 = 2;
pub const WINAPI_FAMILY: u32 = 100;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const __SAL_H_FULL_VER: u32 = 140050727;
pub const __SPECSTRINGS_STRICT_LEVEL: u32 = 1;
pub const __drv_typeConst: u32 = 0;
pub const __drv_typeCond: u32 = 1;
pub const __drv_typeBitset: u32 = 2;
pub const __drv_typeExpr: u32 = 3;
pub const SHORT_MIN: i32 = -32768;
pub const INT_MIN: i32 = -2147483648;
pub const LONG_MIN: i32 = -2147483648;
pub const BYTE_MAX: u32 = 255;
pub const SHORT_MAX: u32 = 32767;
pub const USHORT_MAX: u32 = 65535;
pub const WORD_MAX: u32 = 65535;
pub const INT_MAX: u32 = 2147483647;
pub const UINT_MAX: u32 = 4294967295;
pub const LONG_MAX: u32 = 2147483647;
pub const ULONG_MAX: u32 = 4294967295;
pub const DWORD_MAX: u32 = 4294967295;
pub const BYTE_ERROR: u32 = 255;
pub const SHORT_ERROR: i32 = -1;
pub const USHORT_ERROR: u32 = 65535;
pub const WORD_ERROR: u32 = 65535;
pub const INT_ERROR: i32 = -1;
pub const UINT_ERROR: u32 = 4294967295;
pub const LONG_ERROR: i32 = -1;
pub const ULONG_ERROR: u32 = 4294967295;
pub const DWORD_ERROR: u32 = 4294967295;
pub const SIM_ANY: u32 = 0;
pub const SIM_FS98: u32 = 1;
pub const SIM_FS2K: u32 = 2;
pub const SIM_CFS2: u32 = 3;
pub const SIM_CFS1: u32 = 4;
pub const SIM_FLY: u32 = 5;
pub const SIM_FS2K2: u32 = 6;
pub const SIM_FS2K4: u32 = 7;
pub const SIM_FSX: u32 = 8;
pub const SIM_ESP: u32 = 9;
pub const SIM_P3D: u32 = 10;
pub const SIM_FSX64: u32 = 11;
pub const SIM_P3D64: u32 = 12;
pub const FSUIPC_ERR_OK: u32 = 0;
pub const FSUIPC_ERR_OPEN: u32 = 1;
pub const FSUIPC_ERR_NOFS: u32 = 2;
pub const FSUIPC_ERR_REGMSG: u32 = 3;
pub const FSUIPC_ERR_ATOM: u32 = 4;
pub const FSUIPC_ERR_MAP: u32 = 5;
pub const FSUIPC_ERR_VIEW: u32 = 6;
pub const FSUIPC_ERR_VERSION: u32 = 7;
pub const FSUIPC_ERR_WRONGFS: u32 = 8;
pub const FSUIPC_ERR_NOTOPEN: u32 = 9;
pub const FSUIPC_ERR_NODATA: u32 = 10;
pub const FSUIPC_ERR_TIMEOUT: u32 = 11;
pub const FSUIPC_ERR_SENDMSG: u32 = 12;
pub const FSUIPC_ERR_DATA: u32 = 13;
pub const FSUIPC_ERR_RUNNING: u32 = 14;
pub const FSUIPC_ERR_SIZE: u32 = 15;
pub type CHAR = ::std::os::raw::c_char;
pub type INT8 = ::std::os::raw::c_schar;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type BYTE = ::std::os::raw::c_uchar;
pub type SHORT = ::std::os::raw::c_short;
pub type INT16 = ::std::os::raw::c_short;
pub type USHORT = ::std::os::raw::c_ushort;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type WORD = ::std::os::raw::c_ushort;
pub type INT = ::std::os::raw::c_int;
pub type INT32 = ::std::os::raw::c_int;
pub type UINT = ::std::os::raw::c_uint;
pub type UINT32 = ::std::os::raw::c_uint;
pub type LONG = ::std::os::raw::c_long;
pub type ULONG = ::std::os::raw::c_ulong;
pub type DWORD = ::std::os::raw::c_ulong;
pub type LONGLONG = ::std::os::raw::c_longlong;
pub type LONG64 = ::std::os::raw::c_longlong;
pub type INT64 = ::std::os::raw::c_longlong;
pub type ULONGLONG = ::std::os::raw::c_ulonglong;
pub type DWORDLONG = ::std::os::raw::c_ulonglong;
pub type ULONG64 = ::std::os::raw::c_ulonglong;
pub type DWORD64 = ::std::os::raw::c_ulonglong;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type INT_PTR = ::std::os::raw::c_longlong;
pub type UINT_PTR = ::std::os::raw::c_ulonglong;
pub type LONG_PTR = ::std::os::raw::c_longlong;
pub type ULONG_PTR = ::std::os::raw::c_ulonglong;
pub type DWORD_PTR = ULONG_PTR;
pub type SSIZE_T = LONG_PTR;
pub type SIZE_T = ULONG_PTR;
pub type HRESULT = ::std::os::raw::c_long;
pub type __C_ASSERT__ = [::std::os::raw::c_char; 1usize];
pub type BOOL = ::std::os::raw::c_int;
extern "C" {
    pub static mut FSUIPC_Version: DWORD;
}
extern "C" {
    pub static mut FSUIPC_FS_Version: DWORD;
}
extern "C" {
    pub static mut FSUIPC_Lib_Version: DWORD;
}
extern "C" {
    pub fn FSUIPC_Open(dwFSReq: DWORD, pdwResult: *mut DWORD) -> BOOL;
}
extern "C" {
    pub fn FSUIPC_Close();
}
extern "C" {
    pub fn FSUIPC_Read(
        dwOffset: DWORD,
        dwSize: DWORD,
        pDest: *mut ::std::os::raw::c_void,
        pdwResult: *mut DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn FSUIPC_ReadSpecial(
        dwOffset: DWORD,
        dwSize: DWORD,
        pDest: *mut ::std::os::raw::c_void,
        pdwResult: *mut DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn FSUIPC_Write(
        dwOffset: DWORD,
        dwSize: DWORD,
        pSrce: *mut ::std::os::raw::c_void,
        pdwResult: *mut DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn FSUIPC_Process(pdwResult: *mut DWORD) -> BOOL;
}
