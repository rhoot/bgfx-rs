// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate glutin;
extern crate libc;

use bgfx::{Bgfx, PlatformData, RenderFrame};

use glutin::{Api, GlContext, GlRequest, Window, WindowBuilder};

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

/// Events received by the main thread, sent by the render thread.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
    /// Window close event.
    Close,

    /// Window size event.
    Size(u16, u16),
}

/// Example application.
pub struct EventQueue {
    /// Receiver for events from the render thread.
    event_rx: Receiver<Event>,
}

impl EventQueue {

    /// Handles events received from the render thread. If there are no events to process, returns
    /// instantly.
    ///
    /// Returns `true` if the app should exit.
    pub fn handle_events(&self,
                         bgfx: &Bgfx,
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
pub fn load_program<'a, 'b>(bgfx: &'a Bgfx,
                            vsh_name: &'b str,
                            fsh_name: &'b str)
                            -> bgfx::Program<'a> {
    let renderer = bgfx.get_renderer_type();
    let exe_path = env::current_exe().unwrap();
    let exe_stem = exe_path.file_stem().unwrap();
    let assets_path = format!("examples/assets/{}", exe_stem.to_str().unwrap());
    let vsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, vsh_name);
    let fsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, fsh_name);
    let vsh_mem = bgfx::Memory::copy(bgfx, &load_file(&vsh_path));
    let fsh_mem = bgfx::Memory::copy(bgfx, &load_file(&fsh_path));
    let vsh = bgfx::Shader::new(vsh_mem);
    let fsh = bgfx::Shader::new(fsh_mem);

    bgfx::Program::new(vsh, fsh)
}

/// Returns a new `bgfx::Application`.
///
/// # Arguments
///
/// * `window` - Reference to the glutin window object.
#[cfg(target_os = "linux")]
fn init_bgfx_platform(window: &Window) {
    use glutin::os::unix::WindowExt;

    PlatformData::new()
        .display(window.get_xlib_display().unwrap())
        .window(window.get_xlib_window().unwrap())
        .apply()
        .unwrap();
}

#[cfg(target_os = "windows")]
fn init_bgfx_platform(window: &Window) {
    use glutin::os::windows::WindowExt;

    PlatformData::new()
        .window(window.get_hwnd())
        .apply()
        .unwrap();
}

pub fn run_example<M>(width: u16, height: u16, main: M)
    where M: Send + 'static + FnOnce(EventQueue)
{
    let window = WindowBuilder::new()
                     .with_dimensions(width as u32, height as u32)
                     .with_gl(GlRequest::Specific(Api::OpenGl, (3, 1)))
                     .with_title(String::from("BGFX"))
                     .build()
                     .expect("Failed to create window");

    unsafe {
        window.make_current().unwrap();
    }

    // Create the channel used for communication between the main and render threads.
    let (event_tx, event_rx) = channel::<Event>();

    // Initialize the example data.
    let mut data = ExampleData {
        should_close: false,
        window: window,
        event_tx: event_tx,
    };

    init_bgfx_platform(&data.window);

    bgfx::render_frame();

    thread::spawn(move || {
        main(EventQueue { event_rx: event_rx });
    });

    while !data.process_events() {
        bgfx::render_frame();
    }

    while bgfx::render_frame() != RenderFrame::NoContext {
        thread::sleep(Duration::from_millis(1));
    }
}
