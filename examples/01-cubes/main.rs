extern crate bgfx;
extern crate common;

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

#[repr(packed)]
//#[rustfmt_skip]
const CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

#[repr(packed)]
//#[rustfmt_skip]
const CUBE_INDICES: [u32; 36] = [
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
    debug: bgfx::DebugFlags,
    reset: bgfx::ResetFlags,
    decl: bgfx::VertexDecl,
    vbh: bgfx::VertexBuffer<'a>,
    ibh: bgfx::IndexBuffer<'a>,
    program: bgfx::Program<'a>,
}

impl<'a> Cubes<'a> {
    pub fn new(bgfx: &'a mut bgfx::MainContext, example: &'a common::Example) -> Cubes<'a> {
        let width: u16 = 1280;
        let height: u16 = 720;
        let debug = bgfx::DEBUG_TEXT;
        let reset = bgfx::RESET_VSYNC;

        bgfx.init(None, None, None);
        bgfx.reset(width, height, reset);

        // Enable debug text.
        bgfx.set_debug(debug);

        // Set view 0 clear state.
        let clear_flags = bgfx::CLEAR_COLOR | bgfx::CLEAR_DEPTH;
        bgfx.set_view_clear(0, clear_flags, 0x303030ff, 1.0_f32, 0);

        // Create vertex stream declaration
        let decl = PosColorVertex::build_decl();

        // Create static vertex buffer.
        let vbh = bgfx::VertexBuffer::new(bgfx, &CUBE_VERTICES, &decl, bgfx::BUFFER_NONE);

        // Create static index buffer.
        let ibh = bgfx::IndexBuffer::new(bgfx, &CUBE_INDICES, bgfx::BUFFER_NONE);

        // Create program from shaders.
        let program = common::load_program(bgfx, "vs_cubes.sc", "fs_cubes.sc");

        Cubes {
            bgfx: bgfx,
            example: example,
            width: 1280,
            height: 720,
            debug: bgfx::DEBUG_TEXT,
            reset: bgfx::RESET_VSYNC,
            decl: PosColorVertex::build_decl(),
            vbh: vbh,
            ibh: ibh,
            program: program,
        }
    }

    pub fn shutdown(&mut self) {

    }

    pub fn update(&mut self) -> bool {
        false
    }
}

fn example(bgfx: &mut bgfx::MainContext, example: &common::Example) {
    let mut cubes = Cubes::new(bgfx, example);
    while cubes.update() {}
    cubes.shutdown();
}

fn main() {
    common::run_example(1024, 768, example);
}
