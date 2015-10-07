extern crate bgfx;
extern crate glfw;
extern crate libc;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use glfw::{Context, Glfw, Window, WindowEvent};

/// Events received by the main thread, sent by the render thread.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {

    /// Window close event.
    Close,

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
    pub fn handle_events(&self) -> bool {
        let mut close = false;

        let result = self.event_rx.try_recv();

        if result.is_ok() {
            match result.ok().unwrap() {
                Event::Close => close = true,
            }
        }

        close
    }

}

/// Example data used by the render thread.
struct ExampleData {

    /// The 'Glfw' object.
    glfw: Glfw,

    /// Receiver of window events from GLFW.
    events: Receiver<(f64, WindowEvent)>,

    /// The GLFW window object.
    window: Window,

    /// Sender of events to the main thread.
    event_tx: Sender<Event>,

}

impl ExampleData {

    /// Process GLFW events, and potentially forward them to the main thread.
    ///
    /// Returns `true` if the example should exit.
    fn process_events(&mut self) -> bool {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => { self.event_tx.send(Event::Close).unwrap(); },
                ref e              => { panic!(format!("Unhandled event: {:?}", e)) },
            }
        }

        self.window.should_close()
    }

}

/// Returns a new `bgfx::Application`.
///
/// # Arguments
///
/// * `glfw` - Reference to the `Glfw` object.
/// * `window` - Reference to the GLFW window object.
fn create_bgfx_app(glfw: &Glfw, window: &Window) -> bgfx::Application {
    if cfg!(target_os = "linux") {
        return bgfx::create(
            glfw.get_x11_display(),
            window.get_x11_window(),
            window.get_glx_context()
        );
    }

    unreachable!()
}

/// Runs an example.
///
/// # Arguments
///
/// * `width` - Initial width of the window, in pixels.
/// * `height` - Initial height of the window, in pixels.
/// * `main` - Function to act as the entry point for the example.
pub fn run_example<M>(width: u32, height: u32, main: M) where
    M : Send + 'static + FnOnce(&bgfx::MainContext, &Example)
{
    // Initialize GLFW and create the window.
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(width, height, "BGFX", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_close_polling(true);
    window.make_current();

    // Create the channel used for communication between the main and render threads.
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    // Initialize the example.
    let mut data = ExampleData {
        glfw: glfw,
        events: events,
        window: window,
        event_tx: event_tx,
    };

    let bgfx_app = create_bgfx_app(&data.glfw, &data.window);

    // Main thread implementation.
    let main_thread = move |bgfx: &bgfx::MainContext| {
        let example = Example {
            event_rx: event_rx,
        };
        main(bgfx, &example);
    };

    // Render thread implementation.
    let render_thread = |bgfx: &bgfx::RenderContext| {
        while !data.process_events() {
            bgfx.render_frame();
        }
    };

    // Run the application
    bgfx_app.run(main_thread, render_thread);
}