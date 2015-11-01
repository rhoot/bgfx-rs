extern crate bgfx;
extern crate common;
extern crate time;

use time::PreciseTime;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

impl PosColorVertex {
    fn build_decl() -> bgfx::VertexDecl {
        let decl = bgfx::VertexDecl::new(None)
                       .add(bgfx::Attrib::Position, 3, bgfx::AttribType::Float, None, None)
                       .add(bgfx::Attrib::Color0, 4, bgfx::AttribType::UInt8, Some(true), None)
                       .end();

        decl
    }
}

//#[rustfmt_skip]
static CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

//#[rustfmt_skip]
static CUBE_INDICES: [u16; 36] = [
    0, 1, 2, // 0
    1, 3, 2,
    4, 6, 5, // 2
    5, 6, 7,
    0, 2, 4, // 4
    4, 2, 6,
    1, 5, 3, // 6
    5, 7, 3,
    0, 4, 1, // 8
    4, 5, 1,
    2, 3, 6, // 10
    6, 3, 7,
];

struct Cubes<'a> {
    bgfx: &'a bgfx::MainContext,
    example: &'a common::Example,
    width: u16,
    height: u16,
    reset: bgfx::ResetFlags,
    vbh: bgfx::VertexBuffer<'a>,
    ibh: bgfx::IndexBuffer<'a>,
    program: bgfx::Program<'a>,
    time: time::PreciseTime,
    last: time::PreciseTime,
}

impl<'a> Cubes<'a> {
    pub fn init(bgfx: &'a mut bgfx::MainContext, example: &'a common::Example) -> Cubes<'a> {
        let width: u16 = 1280;
        let height: u16 = 720;
        let debug = bgfx::DEBUG_TEXT;
        let reset = bgfx::RESET_VSYNC;

        bgfx.init(common::get_renderer_type(), None, None);
        bgfx.reset(width, height, reset);

        // Enable debug text.
        bgfx.set_debug(debug);

        // Set view 0 clear state.
        let clear_flags = bgfx::CLEAR_COLOR | bgfx::CLEAR_DEPTH;
        bgfx.set_view_clear(0, clear_flags, 0x303030ff, 1.0_f32, 0);

        // Create vertex stream declaration
        let decl = PosColorVertex::build_decl();

        // Create static vertex buffer.
        let vbh = bgfx::VertexBuffer::new(bgfx,
                                          bgfx::Memory::reference(&CUBE_VERTICES),
                                          &decl,
                                          bgfx::BUFFER_NONE);

        // Create static index buffer.
        let ibh = bgfx::IndexBuffer::new(bgfx,
                                         bgfx::Memory::reference(&CUBE_INDICES),
                                         bgfx::BUFFER_NONE);

        // Create program from shaders.
        let program = common::load_program(bgfx, "vs_cubes", "fs_cubes");

        let now = PreciseTime::now();

        Cubes {
            bgfx: bgfx,
            example: example,
            width: 1280,
            height: 720,
            reset: reset,
            vbh: vbh,
            ibh: ibh,
            program: program,
            time: now,
            last: now,
        }
    }

    pub fn shutdown(&mut self) {
        // We don't really need to do anything here, the objects will clean themselves up once they
        // go out of scope. This function is really only here to keep the examples similar in
        // structure to the C++ examples.
    }

    pub fn update(&mut self) -> bool {
        if !self.example.handle_events(self.bgfx, &mut self.width, &mut self.height, self.reset) {
            let now = PreciseTime::now();
            let frame_time = self.last.to(now);
            self.last = now;

            let time = (self.time.to(now).num_microseconds().unwrap() as f64) / 1_000_000.0_f64;

            // Use debug font to print information about this example.
            let frame_info = format!("Frame: {:7.3}[ms]", frame_time.num_milliseconds());
            self.bgfx.dbg_text_clear(None, None);
            self.bgfx.dbg_text_print(0, 1, 0x4f, "bgfx/examples/01-cubes");
            self.bgfx.dbg_text_print(0, 2, 0x6f, "Description: Rendering simple static mesh.");
            self.bgfx.dbg_text_print(0, 3, 0x0f, &frame_info);

            let at: [f32; 3] = [0.0, 0.0, 0.0];
            let eye: [f32; 3] = [0.0, 0.0, -35.0];

            // TODO: Support for HMD rendering

            // Set view and projection matrix for view 0.
            let aspect = (self.width as f32) / (self.height as f32);
            let view = bgfx::mtx_look_at(&eye, &at);
            let proj = bgfx::mtx_proj(60.0, aspect, 0.1, 100.0);
            self.bgfx.set_view_transform(0, &view, &proj);

            // Set view 0 default viewport.
            self.bgfx.set_view_rect(0, 0, 0, self.width, self.height);

            // This dummy draw call is here to make sure that view 0 is cleared if no other draw
            // calls are submitted to view 0.
            self.bgfx.touch(0);

            // Submit 11x11 cubes
            for yy in 0..11 {
                for xx in 0..11 {
                    let mut mtx = bgfx::mtx_rotate_xy((time / 0.21) as f32, (time / 0.37) as f32);
                    mtx[12] = -15.0 + (xx as f32) * 3.0;
                    mtx[13] = -15.0 + (yy as f32) * 3.0;
                    mtx[14] = 0.0;

                    // Set model matrix for rendering.
                    self.bgfx.set_transform(&mtx);

                    // Set vertex and index buffer.
                    self.bgfx.set_vertex_buffer(&self.vbh);
                    self.bgfx.set_index_buffer(&self.ibh);

                    // Set render states.
                    self.bgfx.set_state(bgfx::State::new(), None);

                    // Submit primitive for rendering to view 0.
                    self.bgfx.submit(0, &self.program);
                }
            }

            // Advance to next frame. Rendering thread will be kicked to process submitted
            // rendering primitives.
            self.bgfx.frame();

            true
        } else {
            false
        }
    }
}

fn example(bgfx: &mut bgfx::MainContext, example: &common::Example) {
    let mut cubes = Cubes::init(bgfx, example);
    while cubes.update() {}
    cubes.shutdown();
}

fn main() {
    common::run_example(1280, 720, example);
}
