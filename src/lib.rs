extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;

use std::mem;
use std::option::Option;
use std::ptr;
use std::thread;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug)]
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

#[repr(u32)]
#[derive(PartialEq, Eq, Debug)]
pub enum RenderFrame {
    NoContext   = bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT,
    Render      = bgfx_sys::BGFX_RENDER_FRAME_RENDER,
    Exiting     = bgfx_sys::BGFX_RENDER_FRAME_EXITING,
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

pub struct MainContext {
    did_init: bool,
}

pub struct RenderContext {
    __: u32,    // This field is purely used to prevent consumers from creating their own instance
}

pub struct Application {
    __: u32,    // This field is purely used to prevent consumers from creating their own instance
}

impl MainContext {
    #[inline]
    pub fn init(&mut self, renderer: Option<RendererType>, vendor_id: Option<u16>, device_id: Option<u16>) -> bool {
        assert!(!self.did_init);

        unsafe {
            let res = bgfx_sys::bgfx_init(
                renderer.unwrap_or(RendererType::Default) as bgfx_sys::bgfx_renderer_type_t,
                vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE),
                device_id.unwrap_or(0_u16),
                ptr::null_mut(),
                ptr::null_mut()
            );

            self.did_init = res != 0;
        }

        self.did_init
    }

    #[inline]
    pub fn reset(&self, width: u16, height: u16, reset: ResetFlags) {
        unsafe {
            bgfx_sys::bgfx_reset(width as u32, height as u32, reset.bits());
        }
    }

    #[inline]
    pub fn set_debug(&self, debug: DebugFlags) {
        unsafe {
            bgfx_sys::bgfx_set_debug(debug.bits());
        }
    }

    #[inline]
    pub fn set_view_clear(&self, id: u8, flags: ClearFlags, rgba: u32, depth: f32, stencil: u8) {
        unsafe {
            bgfx_sys::bgfx_set_view_clear(id, flags.bits(), rgba, depth, stencil);
        }
    }

    #[inline]
    pub fn set_view_rect(&self, id: u8, x: u16, y: u16, width: u16, height: u16) {
        unsafe {
            bgfx_sys::bgfx_set_view_rect(id, x, y, width, height);
        }
    }

    #[inline]
    pub fn touch(&self, id: u8) {
        unsafe {
            bgfx_sys::bgfx_touch(id);
        }
    }

    #[inline]
    pub fn dbg_text_clear(&self, attr: Option<u8>, small: Option<bool>) {
        let small = if small.unwrap_or(false) { 1_u8 } else { 0_u8 };
        let attr  = attr.unwrap_or(0);

        unsafe {
            bgfx_sys::bgfx_dbg_text_clear(attr, small);
        }
    }

    #[inline]
    pub fn dbg_text_image(&self, x: u16, y: u16, width: u16, height: u16, data: &[u8], pitch: u16) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_image(x, y, width, height, data.as_ptr() as *const libc::c_void, pitch);
        }
    }

    #[inline]
    pub fn dbg_text_print(&self, x: u16, y: u16, attr: u8, text: &str) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr() as *const i8);
        }
    }

    #[inline]
    pub fn frame(&self) -> u32 {
        unsafe {
            bgfx_sys::bgfx_frame()
        }
    }
}

impl Drop for MainContext {
    fn drop(&mut self) {
        if self.did_init {
            unsafe {
                bgfx_sys::bgfx_shutdown();
            }
        }
    }
}

impl RenderContext {
    #[inline]
    pub fn render_frame(&self) -> RenderFrame {
        unsafe {
            let max = bgfx_sys::BGFX_RENDER_FRAME_COUNT;
            let res = bgfx_sys::bgfx_render_frame();
            assert!(res < max);

            mem::transmute(res)
        }
    }
}

impl Application {
    pub fn run<M, R>(&self, main: M, render: R) where
        M: Send + 'static + FnOnce(&mut MainContext),
        R: FnOnce(&RenderContext)
    {
        // We need to launch the render thread *before* the main thread starts
        // executing things, so let's do it now.
        let ctx = RenderContext { __: 0 };
        ctx.render_frame();

        // Many platforms require rendering to happen on the main thread. With
        // *no* platform is this a problem. As such, we spawn a *new* thread
        // to use as the main thread, and adopt the current one as the render
        // thread.
        let main_thread = thread::spawn(move || {
            let mut ctx = MainContext { did_init: false };
            main(&mut ctx);
        });

        render(&ctx);
        while ctx.render_frame() != RenderFrame::NoContext {
            thread::sleep_ms(1);
        }

        main_thread.join().unwrap();
    }
}

pub fn create(display: *mut libc::c_void, window: *mut libc::c_void, context: *mut libc::c_void) -> Application {
    // TODO: Only allow one instance

    let mut data = bgfx_sys::Struct_bgfx_platform_data {
        ndt: display,
        nwh: window,
        context: context,
        backBuffer: ptr::null_mut(),
        backBufferDS: ptr::null_mut(),
    };

    unsafe {
        bgfx_sys::bgfx_set_platform_data(&mut data);
    }

    Application { __: 0 }
}
