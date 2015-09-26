extern crate bgfx;
extern crate common;

use bgfx::api;
use std::ptr;

fn main() {
    unsafe {
        api::bgfx_init(api::BGFX_RENDERER_TYPE_OPENGL, api::BGFX_PCI_ID_NONE, 0, ptr::null_mut(), ptr::null_mut());
        api::bgfx_shutdown();
    }
}