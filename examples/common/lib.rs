extern crate bgfx;
extern crate glfw;
extern crate libc;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use glfw::{Context, Glfw, Window, WindowEvent};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
    Close,
}

pub struct Example {
    event_rx: Receiver<Event>,
}

impl Example {
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

struct ExampleData {
    glfw: Glfw,
    events: Receiver<(f64, WindowEvent)>,
    window: Window,
    event_tx: Sender<Event>,
}

impl ExampleData {
    pub fn process_events(&mut self) -> bool {
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

#[cfg(target_os = "linux")]
fn create_app(glfw: &Glfw, window: &Window) -> bgfx::Application {
    bgfx::create(
        glfw.get_x11_display(),
        window.get_x11_window(),
        window.get_glx_context()
    )
}

pub fn run_example<M>(width: u32, height: u32, main: M) where
    M : Send + 'static + FnOnce(&bgfx::MainContext, &Example)
{
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(width, height, "BGFX", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_close_polling(true);
    window.make_current();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let mut data = ExampleData {
        glfw: glfw,
        events: events,
        window: window,
        event_tx: event_tx,
    };

    let app = create_app(&data.glfw, &data.window);

    let main_thread = move |bgfx: &bgfx::MainContext| {
        let example = Example {
            event_rx: event_rx,
        };
        main(bgfx, &example);
    };

    let render_thread = |bgfx: &bgfx::RenderContext| {
        while !data.process_events() {
            bgfx.render_frame();
        }
    };

    app.run(main_thread, render_thread);
}