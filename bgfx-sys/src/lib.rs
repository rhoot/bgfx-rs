#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub type __builtin_va_list = ::libc::c_void;
pub type va_list           = ::libc::c_void;
pub type size_t            = ::libc::size_t;

include!("ffi.rs");

pub const BGFX_PCI_ID_NONE: ::libc::c_ushort = 0;