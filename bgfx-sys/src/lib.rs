#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

type __builtin_va_list = ::libc::c_void;
type size_t            = ::libc::size_t;

include!("ffi.rs");

pub const BGFX_PCI_ID_NONE: ::libc::c_ushort = 0;