extern crate bgfx;
extern crate glfw;
extern crate libc;

use bgfx::{Bgfx, BgfxPlatform};
use glfw::{Context, Glfw, Window};

pub struct ExampleData {
    _glfw:     Glfw,
    _window:   Window,
    _platform: BgfxPlatform,
    _bgfx:     Bgfx,
}

#[cfg(target_os="linux")]
fn make_platform(glfw: &Glfw, window: &Window) -> bgfx::BgfxPlatform {
    bgfx::BgfxPlatform::from_glfw(
        glfw.get_x11_display(),
        window.get_x11_window(),
        window.get_glx_context()
    )
}

pub fn init(width: u32, height: u32, title: &str) -> ExampleData {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, _) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut platform = make_platform(&glfw, &window);
    let bgfx = bgfx::init(&mut platform, None, None, None);

    ExampleData {
        _glfw:     glfw,
        _window:   window,
        _platform: platform,
        _bgfx:     bgfx,
    }

}