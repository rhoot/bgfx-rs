// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate glutin;

use std::cmp::max;

mod common;

const LOGO: &'static [u8] = include_bytes!("assets/00-helloworld/logo.bin");

fn example(bgfx: bgfx::MainContext, example: &common::Example) {
    let mut width: u16 = 1024;
    let mut height: u16 = 720;
    let debug = bgfx::DEBUG_TEXT;
    let reset = bgfx::RESET_VSYNC;

    bgfx.reset(width, height, reset);

    // Enable debug text.
    bgfx.set_debug(debug);

    // Set view 0 clear state.
    let clear = bgfx::CLEAR_COLOR | bgfx::CLEAR_DEPTH;
    bgfx.set_view_clear(0, clear, 0x303030ff, 1.0_f32, 0);

    while !example.handle_events(&bgfx, &mut width, &mut height, reset) {
        // Set view 0 default viewport.
        bgfx.set_view_rect(0, 0, 0, width, height);

        // This dummy draw call is here to make sure that view 0 is cleared
        // if no other draw calls are submitted to view 0.
        bgfx.touch(0);

        // Use debug font to print information about this example.
        let x: u16 = max(width / 2 / 8, 20) - 20;
        let y: u16 = max(height / 2 / 16, 6) - 6;
        bgfx.dbg_text_clear(None, None);
        bgfx.dbg_text_image(x, y, 40, 12, LOGO, 160);
        bgfx.dbg_text_print(0, 1, 0x4f, "examples/00-helloworld.rs");
        bgfx.dbg_text_print(0, 2, 0x6f, "Description: Initialization and debug text.");

        // Advance to next frame. Rendering thread will be kicked to
        // process submitted rendering primitives.
        bgfx.frame();
    }

    // bgfx will automatically be shut down when the local `bgfx` binding
    // goes out of scope.
}

fn main() {
    common::run_example(1024, 768, example);
}
