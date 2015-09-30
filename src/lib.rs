extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;

use std::option::Option;
use std::ptr;

#[repr(u32)]
pub enum RendererType {
    Null        = bgfx_sys::BGFX_RENDERER_TYPE_NULL,
    Direct3D9   = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9,
    Direct3D11  = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11,
    Direct3D12  = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12,
    Metal       = bgfx_sys::BGFX_RENDERER_TYPE_METAL,
    OpenGLES    = bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES,
    OpenGL      = bgfx_sys::BGFX_RENDERER_TYPE_OPENGL,
    Vulkan      = bgfx_sys::BGFX_RENDERER_TYPE_VULKAN,
    Default     = bgfx_sys::BGFX_RENDERER_TYPE_COUNT,
}

bitflags! {
    flags ClearFlags: u16 {
        const CLEAR_NONE               = bgfx_sys::BGFX_CLEAR_NONE,
        const CLEAR_COLOR              = bgfx_sys::BGFX_CLEAR_COLOR,
        const CLEAR_DEPTH              = bgfx_sys::BGFX_CLEAR_DEPTH,
        const CLEAR_STENCIL            = bgfx_sys::BGFX_CLEAR_STENCIL,
        const CLEAR_DISCARD_COLOR_0    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_0,
        const CLEAR_DISCARD_COLOR_1    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_1,
        const CLEAR_DISCARD_COLOR_2    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_2,
        const CLEAR_DISCARD_COLOR_3    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_3,
        const CLEAR_DISCARD_COLOR_4    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_4,
        const CLEAR_DISCARD_COLOR_5    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_5,
        const CLEAR_DISCARD_COLOR_6    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_6,
        const CLEAR_DISCARD_COLOR_7    = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_7,
        const CLEAR_DISCARD_DEPTH      = bgfx_sys::BGFX_CLEAR_DISCARD_DEPTH,
        const CLEAR_DISCARD_STENCIL    = bgfx_sys::BGFX_CLEAR_DISCARD_STENCIL,
        const CLEAR_DISCARD_COLOR_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_MASK,
        const CLEAR_DISCARD_MASK       = bgfx_sys::BGFX_CLEAR_DISCARD_MASK,
    }
}

bitflags! {
    flags DebugFlags: u32 {
        const DEBUG_NONE      = bgfx_sys::BGFX_DEBUG_NONE,
        const DEBUG_WIREFRAME = bgfx_sys::BGFX_DEBUG_WIREFRAME,
        const DEBUG_IFH       = bgfx_sys::BGFX_DEBUG_IFH,
        const DEBUG_STATS     = bgfx_sys::BGFX_DEBUG_STATS,
        const DEBUG_TEXT      = bgfx_sys::BGFX_DEBUG_TEXT,
    }
}

bitflags! {
    flags ResetFlags: u32 {
        const RESET_NONE               = bgfx_sys::BGFX_RESET_NONE,
        const RESET_FULLSCREEN         = bgfx_sys::BGFX_RESET_FULLSCREEN,
        const RESET_FULLSCREEN_SHIFT   = bgfx_sys::BGFX_RESET_FULLSCREEN_SHIFT,
        const RESET_FULLSCREEN_MASK    = bgfx_sys::BGFX_RESET_FULLSCREEN_MASK,
        const RESET_MSAA_X2            = bgfx_sys::BGFX_RESET_MSAA_X2,
        const RESET_MSAA_X4            = bgfx_sys::BGFX_RESET_MSAA_X4,
        const RESET_MSAA_X8            = bgfx_sys::BGFX_RESET_MSAA_X8,
        const RESET_MSAA_X16           = bgfx_sys::BGFX_RESET_MSAA_X16,
        const RESET_MSAA_SHIFT         = bgfx_sys::BGFX_RESET_MSAA_SHIFT,
        const RESET_MSAA_MASK          = bgfx_sys::BGFX_RESET_MSAA_MASK,
        const RESET_VSYNC              = bgfx_sys::BGFX_RESET_VSYNC,
        const RESET_MAXANISOTROPY      = bgfx_sys::BGFX_RESET_MAXANISOTROPY,
        const RESET_CAPTURE            = bgfx_sys::BGFX_RESET_CAPTURE,
        const RESET_HMD                = bgfx_sys::BGFX_RESET_HMD,
        const RESET_HMD_DEBUG          = bgfx_sys::BGFX_RESET_HMD_DEBUG,
        const RESET_HMD_RECENTER       = bgfx_sys::BGFX_RESET_HMD_RECENTER,
        const RESET_FLUSH_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLUSH_AFTER_RENDER,
        const RESET_FLIP_AFTER_RENDER  = bgfx_sys::BGFX_RESET_FLIP_AFTER_RENDER,
        const RESET_SRGB_BACKBUFFER    = bgfx_sys::BGFX_RESET_SRGB_BACKBUFFER,
        const RESET_HIDPI              = bgfx_sys::BGFX_RESET_HIDPI,
    }
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

impl Bgfx {
    pub fn reset(&self, width: u32, height: u32, reset: ResetFlags) {
        unsafe {
            bgfx_sys::bgfx_reset(width, height, reset.bits());
        }
    }

    pub fn set_debug(&self, debug: DebugFlags) {
        unsafe {
            bgfx_sys::bgfx_set_debug(debug.bits());
        }
    }

    pub fn set_view_clear(&self, id: u8, flags: ClearFlags, rgba: u32, depth: f32, stencil: u8) {
        unsafe {
            bgfx_sys::bgfx_set_view_clear(id, flags.bits(), rgba, depth, stencil);
        }
    }

    pub fn set_view_rect(&self, id: u8, x: u16, y: u16, width: u32, height: u32) {
        unsafe {
            bgfx_sys::bgfx_set_view_rect(id, x, y, width as u16, height as u16);
        }
    }

    pub fn touch(&self, id: u8) {
        unsafe {
            bgfx_sys::bgfx_touch(id);
        }
    }

    pub fn dbg_text_clear(&self, attr: Option<u8>, small: Option<bool>) {
        let small = if small.unwrap_or(false) { 1_u8 } else { 0_u8 };
        let attr  = attr.unwrap_or(0);

        unsafe {
            bgfx_sys::bgfx_dbg_text_clear(attr, small);
        }
    }

    pub fn dbg_text_image(&self, x: u16, y: u16, width: u32, height: u32, data: &[u8], pitch: u16) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_image(x, y, width as u16, height as u16, data.as_ptr() as *const libc::c_void, pitch);
        }
    }

    pub fn dbg_text_print(&self, x: u16, y: u16, attr: u8, text: &str) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr() as *const i8);
        }
    }

    pub fn frame(&self) -> u32 {
        unsafe {
            bgfx_sys::bgfx_frame()
        }
    }
}

pub fn init(platform: &mut BgfxPlatform, renderer: Option<RendererType>, vendor_id: Option<u16>, device_id: Option<u16>) -> Bgfx {
    unsafe {
        bgfx_sys::bgfx_set_platform_data(
            &mut platform.data,
        );

        bgfx_sys::bgfx_init(
            renderer.unwrap_or(RendererType::Default) as bgfx_sys::bgfx_renderer_type_t,
            vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE),
            device_id.unwrap_or(0_u16),
            ptr::null_mut(),
            ptr::null_mut()
        );
    }

    Bgfx
}