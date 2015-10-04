extern crate bgfx;
extern crate glfw;
extern crate libc;

use glfw::{Context, Glfw, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Example {
    glfw:   Glfw,
    events: Receiver<(f64, WindowEvent)>,
    window: Window,
}

impl Example {
    pub fn process_events(&mut self, bgfx: &bgfx::RenderContext) -> bool {
        bgfx.render_frame();
        self.glfw.poll_events();

        for _event in glfw::flush_messages(&self.events) {
            // TODO: Handle events... at some point
        }

        self.window.should_close()
    }

    pub fn render_thread(&mut self, bgfx: &bgfx::RenderContext) {
        loop {
            self.process_events(bgfx);
        }
    }
}

#[cfg(target_os = "linux")]
fn make_bgfx(glfw: &Glfw, window: &Window) -> bgfx::Bgfx {
    bgfx::create(
        glfw.get_x11_display(),
        window.get_x11_window(),
        window.get_glx_context()
    )
}

pub fn run_example<M>(width: u32, height: u32, main: M) where
    M : Send + 'static + Fn(&bgfx::MainContext) {

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(width, height, "BGFX", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    let mut example = Example {
        glfw:   glfw,
        events: events,
        window: window,
    };

    let bgfx = make_bgfx(&example.glfw, &example.window);
    bgfx.run(main, |bgfx: &bgfx::RenderContext| {
        example.render_thread(bgfx);
    });
}