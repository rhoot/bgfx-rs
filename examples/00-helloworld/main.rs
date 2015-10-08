extern crate bgfx;
extern crate common;

use std::cmp::max;

const LOGO: &'static [u8] = include_bytes!("logo.bin");

/// Determines the renderer to use based on platform. The sole reason for this to happen instead
/// of using `bgfx::RendererType::Default` is cause `Direct3D12` - which is the default in
/// Windows - currently (2015-10-08) crashes when compiled with MinGW. This is true with the
/// examples in the C++ version of bgfx as well, and not exlusive to Rust.
fn get_renderer_type() -> Option<bgfx::RendererType> {
    if cfg!(windows) && cfg!(target_env = "gnu") {
        Some(bgfx::RendererType::OpenGL)
    } else {
        None
    }
}

fn example(bgfx: &mut bgfx::MainContext, example: &common::Example) {
    let width  = 1024_u16;
    let height = 720_u16;
    let debug  = bgfx::DEBUG_TEXT;
    let reset  = bgfx::RESET_VSYNC;

    bgfx.init(get_renderer_type(), None, None);
    bgfx.reset(width, height, reset);

    // Enable debug text.
    bgfx.set_debug(debug);

    // Set view 0 clear state.
    let clear = bgfx::CLEAR_COLOR | bgfx::CLEAR_DEPTH;
    bgfx.set_view_clear(0, clear, 0x303030ff, 1.0_f32, 0);

    while !example.handle_events() {
        // Set view 0 default viewport.
        bgfx.set_view_rect(0, 0, 0, width, height);

        // This dummy draw call is here to make sure that view 0 is cleared
        // if no other draw calls are submitted to view 0.
        bgfx.touch(0);

        // Use debug font to print information about this example.
        let x = (max(width / 2 / 8, 20) - 20) as u16;
        let y = (max(height / 2 / 16, 6) - 6) as u16;
        bgfx.dbg_text_clear(None, None);
        bgfx.dbg_text_image(x, y, 40, 12, LOGO, 160);
        bgfx.dbg_text_print(0, 1, 0x4f, "bgfx/examples/00-helloworld");
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