extern crate bgfx;
extern crate glfw;
extern crate libc;

use glfw::Context;
use glfw::Glfw;
use glfw::Window;

use std::ptr;

pub struct AppData {
    display: *mut libc::c_void,
    window: *mut libc::c_void,
    context: *mut libc::c_void,
}

impl AppData {

    #[cfg(target_os="linux")]
    fn new(glfw: &Glfw, window: &Window) -> AppData {
        AppData {
            display: glfw.get_x11_display(),
            window: window.get_x11_window(),
            context: window.get_glx_context(),
        }
    }

    #[cfg(target_os="windows")]
    fn new(_glfw: &Glfw, window: &Window) -> AppData {
        AppData {
            display: ptr::null_mut(),
            window: window.get_win32_window(),
            context: ptr::null_mut(),
        }
    }

}

pub fn init(width: u32, height: u32, title: &str) -> AppData {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, _) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut app = AppData::new(glfw, window);
}