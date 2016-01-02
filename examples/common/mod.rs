// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate glutin;
extern crate libc;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use glutin::{Api, GlContext, GlRequest, Window, WindowBuilder};

/// Events received by the main thread, sent by the render thread.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
    /// Window close event.
    Close,

    /// Window size event.
    Size(u16, u16),
}

/// Example application.
pub struct Example {
    /// Receiver for events from the render thread.
    event_rx: Receiver<Event>,
}

impl Example {

    /// Handles events received from the render thread. If there are no events to process, returns
    /// instantly.
    ///
    /// Returns `true` if the app should exit.
    pub fn handle_events(&self,
                         bgfx: &bgfx::MainContext,
                         width: &mut u16,
                         height: &mut u16,
                         reset: bgfx::ResetFlags)
                         -> bool {
        let mut close = false;

        loop {
            let result = self.event_rx.try_recv();

            if !result.is_ok() {
                break;
            }

            match result.ok().unwrap() {
                Event::Close => close = true,
                Event::Size(w, h) => {
                    *width = w;
                    *height = h;
                    bgfx.reset(w, h, reset);
                }
            }
        }


        close
    }

}

/// Example data used by the render thread.
struct ExampleData {
    should_close: bool,

    /// The glutin window object.
    window: Window,

    /// Sender of events to the main thread.
    event_tx: Sender<Event>,
}

impl ExampleData {

    /// Process glutin events, and potentially forward them to the main thread.
    ///
    /// Returns `true` if the example should exit.
    fn process_events(&mut self) -> bool {
        for event in self.window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    self.should_close = true;
                    self.event_tx.send(Event::Close).unwrap();
                }
                glutin::Event::Resized(w, h) => {
                    self.event_tx.send(Event::Size(w as u16, h as u16)).unwrap();
                }
                _ => {}
            }
        }

        self.should_close
    }

}

/// Loads the contents of a file and returns it.
fn load_file(name: &str) -> Vec<u8> {
    let mut data = Vec::new();
    let mut file = File::open(name).unwrap();
    file.read_to_end(&mut data).unwrap();
    data
}

/// Loads the two given shaders from disk, and creates a program using the new
/// shaders.
///
/// # Arguments
///
/// * ``
#[allow(dead_code)]
pub fn load_program<'a, 'b>(bgfx: &'a bgfx::MainContext,
                            vsh_name: &'b str,
                            fsh_name: &'b str)
                            -> bgfx::Program<'a> {
    let renderer = bgfx.get_renderer_type();
    let exe_path = env::current_exe().unwrap();
    let exe_stem = exe_path.file_stem().unwrap();
    let assets_path = format!("examples/assets/{}", exe_stem.to_str().unwrap());
    let vsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, vsh_name);
    let fsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, fsh_name);
    let vsh_mem = bgfx::Memory::copy(&load_file(&vsh_path));
    let fsh_mem = bgfx::Memory::copy(&load_file(&fsh_path));
    let vsh = bgfx::Shader::new(bgfx, vsh_mem);
    let fsh = bgfx::Shader::new(bgfx, fsh_mem);

    bgfx::Program::new(vsh, fsh)
}

/// Returns a new `bgfx::Application`.
///
/// # Arguments
///
/// * `window` - Reference to the glutin window object.
#[cfg(target_os = "linux")]
fn create_bgfx(window: &Window) -> bgfx::Config {
    use glutin::os::unix::WindowExt;
    let mut bgfx = bgfx::create();
    bgfx.display(window.get_xlib_display().unwrap());
    bgfx.window(window.get_xlib_window().unwrap());
    bgfx
}

#[cfg(target_os = "windows")]
fn create_bgfx(window: &Window) -> bgfx::Config {
    use glutin::os::windows::WindowExt;
    let mut bgfx = bgfx::create();
    bgfx.window(window.get_hwnd());
    bgfx
}

/// Runs an example.
///
/// # Arguments
///
/// * `width` - Initial width of the window, in pixels.
/// * `height` - Initial height of the window, in pixels.
/// * `main` - Function to act as the entry point for the example.
pub fn run_example<M>(width: u32, height: u32, main: M)
    where M: Send + 'static + FnOnce(bgfx::MainContext, &Example)
{
    let window = WindowBuilder::new()
        .with_dimensions(width, height)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 1)))
        .with_title(String::from("BGFX"))
        .build()
        .expect("Failed to create window");

    unsafe { window.make_current().unwrap(); }

    // Create the channel used for communication between the main and render threads.
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    // Initialize the example.
    let mut data = ExampleData {
        should_close: false,
        window: window,
        event_tx: event_tx,
    };

    let bgfx = create_bgfx(&data.window);

    // Main thread implementation.
    let main_thread = move |bgfx: bgfx::MainContext| {
        let example = Example { event_rx: event_rx };
        main(bgfx, &example);
    };

    // Render thread implementation.
    let render_thread = |bgfx: bgfx::RenderContext| {
        while !data.process_events() {
            bgfx.render_frame();
        }
    };

    // Run the application
    bgfx.run(main_thread, render_thread).unwrap();
}
