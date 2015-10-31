#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub type va_list  = ::libc::c_void;
pub type size_t   = ::libc::size_t;
pub type int32_t  = i32;
pub type uint8_t  = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

include!("ffi_bgfx.rs");
include!("ffi_bgfxplatform.rs");
include!("ffi_bx.rs");

pub const BGFX_PCI_ID_NONE:                 u16 = 0;

pub const BGFX_CLEAR_NONE:                  u16 = 0x0000;
pub const BGFX_CLEAR_COLOR:                 u16 = 0x0001;
pub const BGFX_CLEAR_DEPTH:                 u16 = 0x0002;
pub const BGFX_CLEAR_STENCIL:               u16 = 0x0004;
pub const BGFX_CLEAR_DISCARD_COLOR_0:       u16 = 0x0008;
pub const BGFX_CLEAR_DISCARD_COLOR_1:       u16 = 0x0010;
pub const BGFX_CLEAR_DISCARD_COLOR_2:       u16 = 0x0020;
pub const BGFX_CLEAR_DISCARD_COLOR_3:       u16 = 0x0040;
pub const BGFX_CLEAR_DISCARD_COLOR_4:       u16 = 0x0080;
pub const BGFX_CLEAR_DISCARD_COLOR_5:       u16 = 0x0100;
pub const BGFX_CLEAR_DISCARD_COLOR_6:       u16 = 0x0200;
pub const BGFX_CLEAR_DISCARD_COLOR_7:       u16 = 0x0400;
pub const BGFX_CLEAR_DISCARD_DEPTH:         u16 = 0x0800;
pub const BGFX_CLEAR_DISCARD_STENCIL:       u16 = 0x1000;

pub const BGFX_CLEAR_DISCARD_COLOR_MASK:    u16 =
    (
        BGFX_CLEAR_DISCARD_COLOR_0 |
        BGFX_CLEAR_DISCARD_COLOR_1 |
        BGFX_CLEAR_DISCARD_COLOR_2 |
        BGFX_CLEAR_DISCARD_COLOR_3 |
        BGFX_CLEAR_DISCARD_COLOR_4 |
        BGFX_CLEAR_DISCARD_COLOR_5 |
        BGFX_CLEAR_DISCARD_COLOR_6 |
        BGFX_CLEAR_DISCARD_COLOR_7
    );

pub const BGFX_CLEAR_DISCARD_MASK:          u16 =
    (
        BGFX_CLEAR_DISCARD_COLOR_MASK |
        BGFX_CLEAR_DISCARD_DEPTH |
        BGFX_CLEAR_DISCARD_STENCIL
    );

pub const BGFX_DEBUG_NONE:                  u32 = 0x00000000;
pub const BGFX_DEBUG_WIREFRAME:             u32 = 0x00000001;
pub const BGFX_DEBUG_IFH:                   u32 = 0x00000002;
pub const BGFX_DEBUG_STATS:                 u32 = 0x00000004;
pub const BGFX_DEBUG_TEXT:                  u32 = 0x00000008;

pub const BGFX_RESET_NONE:                  u32 = 0x00000000;
pub const BGFX_RESET_FULLSCREEN:            u32 = 0x00000001;
pub const BGFX_RESET_FULLSCREEN_SHIFT:      u32 = 0;
pub const BGFX_RESET_FULLSCREEN_MASK:       u32 = 0x00000001;
pub const BGFX_RESET_MSAA_X2:               u32 = 0x00000010;
pub const BGFX_RESET_MSAA_X4:               u32 = 0x00000020;
pub const BGFX_RESET_MSAA_X8:               u32 = 0x00000030;
pub const BGFX_RESET_MSAA_X16:              u32 = 0x00000040;
pub const BGFX_RESET_MSAA_SHIFT:            u32 = 4;
pub const BGFX_RESET_MSAA_MASK:             u32 = 0x00000070;
pub const BGFX_RESET_VSYNC:                 u32 = 0x00000080;
pub const BGFX_RESET_MAXANISOTROPY:         u32 = 0x00000100;
pub const BGFX_RESET_CAPTURE:               u32 = 0x00000200;
pub const BGFX_RESET_HMD:                   u32 = 0x00000400;
pub const BGFX_RESET_HMD_DEBUG:             u32 = 0x00000800;
pub const BGFX_RESET_HMD_RECENTER:          u32 = 0x00001000;
pub const BGFX_RESET_FLUSH_AFTER_RENDER:    u32 = 0x00002000;
pub const BGFX_RESET_FLIP_AFTER_RENDER:     u32 = 0x00004000;
pub const BGFX_RESET_SRGB_BACKBUFFER:       u32 = 0x00008000;
pub const BGFX_RESET_HIDPI:                 u32 = 0x00010000;

pub const BGFX_BUFFER_NONE:                 u16 = 0x0000;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X1:   u16 = 0x0001;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X2:   u16 = 0x0002;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X4:   u16 = 0x0003;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X1:  u16 = 0x0004;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X2:  u16 = 0x0005;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X4:  u16 = 0x0006;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X1:  u16 = 0x0007;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X2:  u16 = 0x0008;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X4:  u16 = 0x0009;
pub const BGFX_BUFFER_COMPUTE_FORMAT_SHIFT: u16 = 0;
pub const BGFX_BUFFER_COMPUTE_FORMAT_MASK:  u16 = 0x000f;
pub const BGFX_BUFFER_COMPUTE_TYPE_UINT:    u16 = 0x0010;
pub const BGFX_BUFFER_COMPUTE_TYPE_INT:     u16 = 0x0020;
pub const BGFX_BUFFER_COMPUTE_TYPE_FLOAT:   u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_TYPE_SHIFT:   u16 = 4;
pub const BGFX_BUFFER_COMPUTE_TYPE_MASK:    u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_READ:         u16 = 0x0100;
pub const BGFX_BUFFER_COMPUTE_WRITE:        u16 = 0x0200;
pub const BGFX_BUFFER_DRAW_INDIRECT:        u16 = 0x0400;
pub const BGFX_BUFFER_ALLOW_RESIZE:         u16 = 0x0800;
pub const BGFX_BUFFER_INDEX32:              u16 = 0x1000;

pub const BGFX_BUFFER_COMPUTE_READ_WRITE:   u16 =
    (
        BGFX_BUFFER_COMPUTE_READ |
        BGFX_BUFFER_COMPUTE_WRITE
    );
