extern crate bgfx_sys;
extern crate libc;

use std::option::Option;
use std::ptr;

pub enum RendererType {
    Null        = bgfx_sys::BGFX_RENDERER_TYPE_NULL as isize,
    Direct3D9   = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9 as isize,
    Direct3D11  = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11 as isize,
    Direct3D12  = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12 as isize,
    Metal       = bgfx_sys::BGFX_RENDERER_TYPE_METAL as isize,
    OpenGLES    = bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES as isize,
    OpenGL      = bgfx_sys::BGFX_RENDERER_TYPE_OPENGL as isize,
    Vulkan      = bgfx_sys::BGFX_RENDERER_TYPE_VULKAN as isize,
    Default     = bgfx_sys::BGFX_RENDERER_TYPE_COUNT as isize,
}

pub struct BgfxPlatform {
    data: bgfx_sys::Struct_bgfx_platform_data,
}

impl BgfxPlatform {
    pub fn from_glfw(display: *mut libc::c_void, window: *mut libc::c_void, context: *mut libc::c_void) -> BgfxPlatform {
        BgfxPlatform {
            data: bgfx_sys::Struct_bgfx_platform_data {
                ndt:          display,
                nwh:          window,
                context:      context,
                backBuffer:   ptr::null_mut(),
                backBufferDS: ptr::null_mut(),
            },
        }
    }
}

pub struct Bgfx;

impl Drop for Bgfx {
    fn drop(&mut self) {
        unsafe {
            bgfx_sys::bgfx_shutdown();
        }
    }
}

pub fn init(platform: &mut BgfxPlatform, renderer: Option<RendererType>, vendor_id: Option<u16>, device_id: Option<u16>) -> Bgfx {
    unsafe {
        bgfx_sys::bgfx_set_platform_data(
            &mut platform.data,
        );

        bgfx_sys::bgfx_init(
            renderer.unwrap_or(RendererType::Default) as libc::c_uint,
            vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE),
            device_id.unwrap_or(0_u16),
            ptr::null_mut(),
            ptr::null_mut()
        );
    }

    Bgfx
}