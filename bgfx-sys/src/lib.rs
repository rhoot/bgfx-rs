#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub type __builtin_va_list  = ::libc::c_void;
pub type va_list            = ::libc::c_void;
pub type size_t             = ::libc::size_t;

pub type int8_t             = ::libc::c_char;
pub type int16_t            = ::libc::c_short;
pub type int32_t            = ::libc::c_int;
pub type int64_t            = ::libc::c_long;
pub type uint8_t            = ::libc::c_uchar;
pub type uint16_t           = ::libc::c_ushort;
pub type uint32_t           = ::libc::c_uint;
pub type uint64_t           = ::libc::c_ulong;
pub type int_least8_t       = ::libc::c_char;
pub type int_least16_t      = ::libc::c_short;
pub type int_least32_t      = ::libc::c_int;
pub type int_least64_t      = ::libc::c_long;
pub type uint_least8_t      = ::libc::c_uchar;
pub type uint_least16_t     = ::libc::c_ushort;
pub type uint_least32_t     = ::libc::c_uint;
pub type uint_least64_t     = ::libc::c_ulong;
pub type int_fast8_t        = ::libc::c_char;
pub type int_fast16_t       = ::libc::c_long;
pub type int_fast32_t       = ::libc::c_long;
pub type int_fast64_t       = ::libc::c_long;
pub type uint_fast8_t       = ::libc::c_uchar;
pub type uint_fast16_t      = ::libc::c_ulong;
pub type uint_fast32_t      = ::libc::c_ulong;
pub type uint_fast64_t      = ::libc::c_ulong;
pub type intptr_t           = ::libc::c_long;
pub type uintptr_t          = ::libc::c_ulong;
pub type intmax_t           = ::libc::c_long;
pub type uintmax_t          = ::libc::c_ulong;

include!("ffi_bgfx.rs");
include!("ffi_bgfxplatform.rs");

pub const BGFX_PCI_ID_NONE: ::libc::c_ushort = 0;