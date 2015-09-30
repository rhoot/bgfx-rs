extern crate bgfx;
extern crate glfw;
extern crate libc;

use bgfx::BgfxPlatform;
use glfw::{Context, Glfw, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Example {
    glfw:         Glfw,
    events:       Receiver<(f64, WindowEvent)>,
    window:       Window,
    pub platform: BgfxPlatform,
}

impl Example {
    pub fn process_events(&mut self, bgfx: &bgfx::Bgfx, width: u32, height: u32, reset: bgfx::ResetFlags) -> bool {
        self.glfw.poll_events();

        for _event in glfw::flush_messages(&self.events) {
            // TODO: Handle events... at some point
        }

        bgfx.reset(width, height, reset);
        self.window.should_close()
    }
}

#[cfg(target_os="linux")]
fn make_platform_internal(glfw: &Glfw, window: &Window) -> bgfx::BgfxPlatform {
    bgfx::BgfxPlatform::from_glfw(
        glfw.get_x11_display(),
        window.get_x11_window(),
        window.get_glx_context()
    )
}

pub fn make_platform(width: u32, height: u32) -> Example {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(width, height, "BGFX", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    let platform = make_platform_internal(&glfw, &window);

    Example {
        glfw:     glfw,
        events:   events,
        window:   window,
        platform: platform,
    }

}