// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate num;

use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::thread;

use num::FromPrimitive;

mod state;

pub use state::State;

/// Renderer type.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RendererType {
    /// Null renderer, nothing is actually rendered but the library acts as if it was.
    Null = bgfx_sys::BGFX_RENDERER_TYPE_NULL,
    /// DirectX 9 renderer. Only available on Windows.
    Direct3D9 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9,
    /// DirectX 11 renderer. Only available on Windows.
    Direct3D11 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11,
    /// DirectX 12 renderer. Only available on Windows.
    Direct3D12 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12,
    /// Metal renderer. Only available on Apple platforms.
    Metal = bgfx_sys::BGFX_RENDERER_TYPE_METAL,
    /// OpenGLES renderer.
    OpenGLES = bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES,
    /// OpenGL renderer.
    OpenGL = bgfx_sys::BGFX_RENDERER_TYPE_OPENGL,
    /// Vulkan renderer.
    Vulkan = bgfx_sys::BGFX_RENDERER_TYPE_VULKAN,
    /// Used to tell bgfx to use whichever renderer makes most sense for the current platform.
    Default = bgfx_sys::BGFX_RENDERER_TYPE_COUNT,
}

impl RendererType {
    fn from_u32(n: u32) -> Option<RendererType> {
        match n {
            bgfx_sys::BGFX_RENDERER_TYPE_NULL => Some(RendererType::Null),
            bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9 => Some(RendererType::Direct3D9),
            bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11 => Some(RendererType::Direct3D11),
            bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12 => Some(RendererType::Direct3D12),
            bgfx_sys::BGFX_RENDERER_TYPE_METAL => Some(RendererType::Metal),
            bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES => Some(RendererType::OpenGLES),
            bgfx_sys::BGFX_RENDERER_TYPE_OPENGL => Some(RendererType::OpenGL),
            bgfx_sys::BGFX_RENDERER_TYPE_VULKAN => Some(RendererType::Vulkan),
            bgfx_sys::BGFX_RENDERER_TYPE_COUNT => Some(RendererType::Default),
            _ => None,
        }
    }
}

/// `render_frame()` result enumeration.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RenderFrame {
    /// No context is available. This usually means the main thread has exited.
    NoContext = bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT,
    /// The render was performed.
    Render = bgfx_sys::BGFX_RENDER_FRAME_RENDER,
    /// The renderer is exiting.
    Exiting = bgfx_sys::BGFX_RENDER_FRAME_EXITING,
}

impl RenderFrame {
    fn from_u32(n: u32) -> Option<RenderFrame> {
        match n {
            bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT => Some(RenderFrame::NoContext),
            bgfx_sys::BGFX_RENDER_FRAME_RENDER => Some(RenderFrame::Render),
            bgfx_sys::BGFX_RENDER_FRAME_EXITING => Some(RenderFrame::Exiting),
            _ => None,
        }
    }
}

/// Shader attribute.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Attrib {
    /// Position attribute.
    Position = bgfx_sys::BGFX_ATTRIB_POSITION,
    /// Normal attribute.
    Normal = bgfx_sys::BGFX_ATTRIB_NORMAL,
    /// Tangent attribute.
    Tangent = bgfx_sys::BGFX_ATTRIB_TANGENT,
    /// Bitangent attribute.
    Bitangent = bgfx_sys::BGFX_ATTRIB_BITANGENT,
    /// Color 0 attribute.
    Color0 = bgfx_sys::BGFX_ATTRIB_COLOR0,
    /// Color 1 attribute.
    Color1 = bgfx_sys::BGFX_ATTRIB_COLOR1,
    /// Index list attribute.
    Indices = bgfx_sys::BGFX_ATTRIB_INDICES,
    /// Bone weight attribute.
    Weight = bgfx_sys::BGFX_ATTRIB_WEIGHT,
    /// Texture coordinate 0 attribute.
    TexCoord0 = bgfx_sys::BGFX_ATTRIB_TEXCOORD0,
    /// Texture coordinate 1 attribute.
    TexCoord1 = bgfx_sys::BGFX_ATTRIB_TEXCOORD1,
    /// Texture coordinate 2 attribute.
    TexCoord2 = bgfx_sys::BGFX_ATTRIB_TEXCOORD2,
    /// Texture coordinate 3 attribute.
    TexCoord3 = bgfx_sys::BGFX_ATTRIB_TEXCOORD3,
    /// Texture coordinate 4 attribute.
    TexCoord4 = bgfx_sys::BGFX_ATTRIB_TEXCOORD4,
    /// Texture coordinate 5 attribute.
    TexCoord5 = bgfx_sys::BGFX_ATTRIB_TEXCOORD5,
    /// Texture coordinate 6 attribute.
    TexCoord6 = bgfx_sys::BGFX_ATTRIB_TEXCOORD6,
    /// Texture coordinate 7 attribute.
    TexCoord7 = bgfx_sys::BGFX_ATTRIB_TEXCOORD7,
}

/// Shader attribute data type.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AttribType {
    /// Unsigned 8-bit integer.
    Uint8 = bgfx_sys::BGFX_ATTRIB_TYPE_UINT8,
    /// Unsigned 10-bit integer.
    Uint10 = bgfx_sys::BGFX_ATTRIB_TYPE_UINT10,
    /// Signed 16-bit integer.
    Int16 = bgfx_sys::BGFX_ATTRIB_TYPE_INT16,
    /// 16-bit float.
    Half = bgfx_sys::BGFX_ATTRIB_TYPE_HALF,
    /// 32-bit float.
    Float = bgfx_sys::BGFX_ATTRIB_TYPE_FLOAT,
}

// Pending rust-lang/rust#24822 being resolved we're stuck with non-documented bit flags.

bitflags! {
    flags BufferFlags: u16 {
        const BUFFER_NONE = bgfx_sys::BGFX_BUFFER_NONE,
        const BUFFER_COMPUTE_FORMAT_8X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X1,
        const BUFFER_COMPUTE_FORMAT_8X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X2,
        const BUFFER_COMPUTE_FORMAT_8X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X4,
        const BUFFER_COMPUTE_FORMAT_16X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X1,
        const BUFFER_COMPUTE_FORMAT_16X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X2,
        const BUFFER_COMPUTE_FORMAT_16X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X4,
        const BUFFER_COMPUTE_FORMAT_32X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X1,
        const BUFFER_COMPUTE_FORMAT_32X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X2,
        const BUFFER_COMPUTE_FORMAT_32X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X4,
        const BUFFER_COMPUTE_FORMAT_SHIFT = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_SHIFT,
        const BUFFER_COMPUTE_FORMAT_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_MASK,
        const BUFFER_COMPUTE_TYPE_UINT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_UINT,
        const BUFFER_COMPUTE_TYPE_INT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_INT,
        const BUFFER_COMPUTE_TYPE_FLOAT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_FLOAT,
        const BUFFER_COMPUTE_TYPE_SHIFT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_SHIFT,
        const BUFFER_COMPUTE_TYPE_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_MASK,
        const BUFFER_COMPUTE_READ = bgfx_sys::BGFX_BUFFER_COMPUTE_READ,
        const BUFFER_COMPUTE_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_WRITE,
        const BUFFER_DRAW_INDIRECT = bgfx_sys::BGFX_BUFFER_DRAW_INDIRECT,
        const BUFFER_ALLOW_RESIZE = bgfx_sys::BGFX_BUFFER_ALLOW_RESIZE,
        const BUFFER_INDEX32 = bgfx_sys::BGFX_BUFFER_INDEX32,
        const BUFFER_COMPUTE_READ_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_READ_WRITE,
    }
}

bitflags! {
    flags ClearFlags: u16 {
        const CLEAR_NONE = bgfx_sys::BGFX_CLEAR_NONE,
        const CLEAR_COLOR = bgfx_sys::BGFX_CLEAR_COLOR,
        const CLEAR_DEPTH = bgfx_sys::BGFX_CLEAR_DEPTH,
        const CLEAR_STENCIL = bgfx_sys::BGFX_CLEAR_STENCIL,
        const CLEAR_DISCARD_COLOR_0 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_0,
        const CLEAR_DISCARD_COLOR_1 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_1,
        const CLEAR_DISCARD_COLOR_2 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_2,
        const CLEAR_DISCARD_COLOR_3 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_3,
        const CLEAR_DISCARD_COLOR_4 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_4,
        const CLEAR_DISCARD_COLOR_5 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_5,
        const CLEAR_DISCARD_COLOR_6 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_6,
        const CLEAR_DISCARD_COLOR_7 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_7,
        const CLEAR_DISCARD_DEPTH = bgfx_sys::BGFX_CLEAR_DISCARD_DEPTH,
        const CLEAR_DISCARD_STENCIL = bgfx_sys::BGFX_CLEAR_DISCARD_STENCIL,
        const CLEAR_DISCARD_COLOR_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_MASK,
        const CLEAR_DISCARD_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_MASK,
    }
}

bitflags! {
    flags DebugFlags: u32 {
        const DEBUG_NONE = bgfx_sys::BGFX_DEBUG_NONE,
        const DEBUG_WIREFRAME = bgfx_sys::BGFX_DEBUG_WIREFRAME,
        const DEBUG_IFH = bgfx_sys::BGFX_DEBUG_IFH,
        const DEBUG_STATS = bgfx_sys::BGFX_DEBUG_STATS,
        const DEBUG_TEXT = bgfx_sys::BGFX_DEBUG_TEXT,
    }
}

bitflags! {
    flags ResetFlags: u32 {
        const RESET_NONE = bgfx_sys::BGFX_RESET_NONE,
        const RESET_FULLSCREEN = bgfx_sys::BGFX_RESET_FULLSCREEN,
        const RESET_FULLSCREEN_SHIFT = bgfx_sys::BGFX_RESET_FULLSCREEN_SHIFT,
        const RESET_FULLSCREEN_MASK = bgfx_sys::BGFX_RESET_FULLSCREEN_MASK,
        const RESET_MSAA_X2 = bgfx_sys::BGFX_RESET_MSAA_X2,
        const RESET_MSAA_X4 = bgfx_sys::BGFX_RESET_MSAA_X4,
        const RESET_MSAA_X8 = bgfx_sys::BGFX_RESET_MSAA_X8,
        const RESET_MSAA_X16 = bgfx_sys::BGFX_RESET_MSAA_X16,
        const RESET_MSAA_SHIFT = bgfx_sys::BGFX_RESET_MSAA_SHIFT,
        const RESET_MSAA_MASK = bgfx_sys::BGFX_RESET_MSAA_MASK,
        const RESET_VSYNC = bgfx_sys::BGFX_RESET_VSYNC,
        const RESET_MAXANISOTROPY = bgfx_sys::BGFX_RESET_MAXANISOTROPY,
        const RESET_CAPTURE = bgfx_sys::BGFX_RESET_CAPTURE,
        const RESET_HMD = bgfx_sys::BGFX_RESET_HMD,
        const RESET_HMD_DEBUG = bgfx_sys::BGFX_RESET_HMD_DEBUG,
        const RESET_HMD_RECENTER = bgfx_sys::BGFX_RESET_HMD_RECENTER,
        const RESET_FLUSH_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLUSH_AFTER_RENDER,
        const RESET_FLIP_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLIP_AFTER_RENDER,
        const RESET_SRGB_BACKBUFFER = bgfx_sys::BGFX_RESET_SRGB_BACKBUFFER,
        const RESET_HIDPI = bgfx_sys::BGFX_RESET_HIDPI,
    }
}

/// A bgfx-managed buffer of memory.
///
/// It can be created by either copying existing data through `copy(...)`, or by referencing
/// existing memory directly through `reference(...)`.
pub struct Memory<'a> {
    handle: *const bgfx_sys::bgfx_memory_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Memory<'a> {
    /// Copies the source data into a new bgfx-managed buffer.
    ///
    /// IMPORTANT: If this buffer is never passed into a bgfx call, the memory will never be freed,
    /// and will leak.
    #[inline]
    pub fn copy<'b, T>(data: &'b [T]) -> Memory<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_copy(data.as_ptr() as *const libc::c_void,
                                             mem::size_of_val(data) as u32);
            Memory { handle: handle, _phantom: PhantomData }
        }
    }

    /// Creates a reference to the source data for passing into bgfx. When using this constructor
    /// over the `copy` call, no copy will be created. Bgfx will read the source memory directly.
    ///
    /// Note that this is only allowed for static memory, as it's the only way we can guarantee
    /// that the memory will still be valid until bgfx has time to read it.
    #[inline]
    pub fn reference<T>(data: &'static [T]) -> Memory<'static> {
        unsafe {
            let handle = bgfx_sys::bgfx_make_ref(data.as_ptr() as *const libc::c_void,
                                                 mem::size_of_val(data) as u32);
            Memory { handle: handle, _phantom: PhantomData }
        }
    }
}

/// Represents a shader program.
///
/// The program holds a vertex shader and a fragment shader.
pub struct Program<'a> {
    handle: bgfx_sys::bgfx_program_handle_t,
    _vsh: Shader<'a>,
    _fsh: Shader<'a>,
}

impl<'a> Program<'a> {
    /// Creates a new program from a vertex shader and a fragment shader. Ownerships of the shaders
    /// are moved to the program.
    #[inline]
    pub fn new(vsh: Shader<'a>, fsh: Shader<'a>) -> Program<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_program(vsh.handle, fsh.handle, 0);
            Program { handle: handle, _vsh: vsh, _fsh: fsh }
        }
    }
}

impl<'a> Drop for Program<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_program(self.handle) }
    }
}

/// Represents a shader.
pub struct Shader<'a> {
    handle: bgfx_sys::bgfx_shader_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> Shader<'a> {
    /// Creates a new shader from bgfx-managed memory.
    #[inline]
    pub fn new(_context: &'a MainContext, data: Memory<'a>) -> Shader<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_shader(data.handle);
            Shader { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for Shader<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_shader(self.handle) }
    }
}

/// A buffer holding vertex indices.
pub struct IndexBuffer<'a> {
    handle: bgfx_sys::bgfx_index_buffer_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> IndexBuffer<'a> {
    /// Creates a new index buffer from bgfx-managed memory.
    #[inline]
    pub fn new(_context: &'a MainContext,
               indices: Memory<'a>,
               flags: BufferFlags)
               -> IndexBuffer<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_index_buffer(indices.handle, flags.bits());
            IndexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for IndexBuffer<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_index_buffer(self.handle) }
    }
}

/// A buffer holding vertices, each representing a point in space.
pub struct VertexBuffer<'a> {
    handle: bgfx_sys::bgfx_vertex_buffer_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> VertexBuffer<'a> {
    /// Creates a new vertex buffer from bgfx-managed memory.
    #[inline]
    pub fn new<'b>(_context: &'a MainContext,
                   verts: Memory<'a>,
                   decl: &'b VertexDecl,
                   flags: BufferFlags)
                   -> VertexBuffer<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_vertex_buffer(verts.handle,
                                                             &decl.decl,
                                                             flags.bits());
            VertexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for VertexBuffer<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_vertex_buffer(self.handle) }
    }
}

/// Describes the structure of a vertex.
pub struct VertexDecl {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDecl {
    /// Creates a new vertex declaration using a builder.
    ///
    /// # Example
    ///
    /// ```
    /// let decl = VertexDecl::new(None)
    ///          .add(bgfx::Attrib::Position, 3, bgfx::AttribType::Float, None, None)
    ///          .add(bgfx::Attrib::Color0, 4, bgfx::AttribType::Uint8, Some(true), None)
    ///          .end();
    /// ```
    #[inline]
    pub fn new(renderer: Option<RendererType>) -> VertexDeclBuilder {
        let renderer = renderer.unwrap_or(RendererType::Null) as bgfx_sys::bgfx_renderer_type_t;

        unsafe {
            let mut descr = VertexDeclBuilder { decl: mem::uninitialized() };
            bgfx_sys::bgfx_vertex_decl_begin(&mut descr.decl, renderer);
            descr
        }
    }
}

/// Builder for `VertexDecl` instances.
pub struct VertexDeclBuilder {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDeclBuilder {
    /// Adds a field to the structure descriptor. See `VertexDecl::new()` for an example.
    #[inline]
    pub fn add(&mut self,
               attrib: Attrib,
               count: u8,
               kind: AttribType,
               normalized: Option<bool>,
               as_int: Option<bool>)
               -> &mut Self {
        unsafe {
            let normalized = if normalized.unwrap_or(false) { 1 } else { 0 };
            let as_int = if as_int.unwrap_or(false) { 1 } else { 0 };

            bgfx_sys::bgfx_vertex_decl_add(&mut self.decl,
                                           attrib as bgfx_sys::bgfx_attrib_t,
                                           count,
                                           kind as bgfx_sys::bgfx_attrib_type_t,
                                           normalized,
                                           as_int);
        }

        self
    }

    /// Indicates a gap in the vertex structure.
    #[inline]
    pub fn skip(&mut self, bytes: u8) -> &mut Self {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_skip(&mut self.decl, bytes);
        }

        self
    }

    /// Finalizes the construction of the `VertexDecl`.
    #[inline]
    pub fn end(&mut self) -> VertexDecl {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_end(&mut self.decl);
        }

        VertexDecl { decl: self.decl }
    }
}

/// Main thread context.
///
/// This will be passed to the callback provided to `Config::run(...)`. Functionality intended
/// to be executed on the main thread is exposed through this object.
pub struct MainContext {
    __: u32, // This field is purely used to prevent API consumers from creating their own instance
}

impl MainContext {
    #[inline]
    fn new() -> Self {
        MainContext { __: 0 }
    }

    /// Resets the graphics device to the given size.
    #[inline]
    pub fn reset(&self, width: u16, height: u16, reset: ResetFlags) {
        unsafe { bgfx_sys::bgfx_reset(width as u32, height as u32, reset.bits()) }
    }

    /// Sets the debug flags in use.
    #[inline]
    pub fn set_debug(&self, debug: DebugFlags) {
        unsafe { bgfx_sys::bgfx_set_debug(debug.bits()) }
    }

    /// Sets the options to use when clearing the given view.
    #[inline]
    pub fn set_view_clear(&self, id: u8, flags: ClearFlags, rgba: u32, depth: f32, stencil: u8) {
        unsafe { bgfx_sys::bgfx_set_view_clear(id, flags.bits(), rgba, depth, stencil) }
    }

    /// Sets the rectangle to display the given view in.
    #[inline]
    pub fn set_view_rect(&self, id: u8, x: u16, y: u16, width: u16, height: u16) {
        unsafe { bgfx_sys::bgfx_set_view_rect(id, x, y, width, height) }
    }

    #[inline]
    pub fn set_view_transform(&self, id: u8, view: &[f32; 16], proj: &[f32; 16]) {
        unsafe {
            bgfx_sys::bgfx_set_view_transform(id,
                                              view.as_ptr() as *const libc::c_void,
                                              proj.as_ptr() as *const libc::c_void)
        }
    }

    /// Touch a view. ( ͡° ͜ʖ ͡°)
    #[inline]
    pub fn touch(&self, id: u8) {
        unsafe {
            bgfx_sys::bgfx_touch(id);
        }
    }

    /// Clears debug text.
    #[inline]
    pub fn dbg_text_clear(&self, attr: Option<u8>, small: Option<bool>) {
        let small = if small.unwrap_or(false) { 1_u8 } else { 0_u8 };
        let attr = attr.unwrap_or(0);

        unsafe { bgfx_sys::bgfx_dbg_text_clear(attr, small) }
    }

    /// Draws an image to the debug text overlay.
    #[inline]
    pub fn dbg_text_image(&self,
                          x: u16,
                          y: u16,
                          width: u16,
                          height: u16,
                          data: &[u8],
                          pitch: u16) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_image(x,
                                          y,
                                          width,
                                          height,
                                          data.as_ptr() as *const libc::c_void,
                                          pitch)
        }
    }

    /// Displays text in the debug text overlay.
    #[inline]
    pub fn dbg_text_print(&self, x: u16, y: u16, attr: u8, text: &str) {
        unsafe { bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr() as *const i8) }
    }

    /// Finish the frame, syncing up with the render thread. Returns an incrementing frame counter.
    #[inline]
    pub fn frame(&self) -> u32 {
        unsafe { bgfx_sys::bgfx_frame() }
    }

    /// Sets the transform for rendering.
    #[inline]
    pub fn set_transform(&self, mtx: &[f32; 16]) {
        unsafe {
            bgfx_sys::bgfx_set_transform(mtx.as_ptr() as *const libc::c_void, 1);
        }
    }

    /// Sets the vertex buffer for rendering.
    #[inline]
    pub fn set_vertex_buffer(&self, vbh: &VertexBuffer) {
        // TODO: How to solve lifetimes...
        unsafe { bgfx_sys::bgfx_set_vertex_buffer(vbh.handle, 0, std::u32::MAX) }
    }

    /// Sets the index buffer for rendering.
    #[inline]
    pub fn set_index_buffer(&self, ibh: &IndexBuffer) {
        // TODO: How to solve lifetimes...
        unsafe { bgfx_sys::bgfx_set_index_buffer(ibh.handle, 0, std::u32::MAX) }
    }

    /// Sets the render state.
    #[inline]
    pub fn set_state(&self, state: State, rgba: Option<u32>) {
        unsafe { bgfx_sys::bgfx_set_state(state.to_bits(), rgba.unwrap_or(0)) }
    }

    #[inline]
    pub fn submit(&self, view: u8, program: &Program) {
        unsafe {
            bgfx_sys::bgfx_submit(view, program.handle, 0);
        }
    }

    #[inline]
    pub fn get_renderer_type(&self) -> RendererType {
        unsafe { RendererType::from_u32(bgfx_sys::bgfx_get_renderer_type()).unwrap() }
    }

}

impl Drop for MainContext {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_shutdown() }
    }
}

/// Render thread context.
///
/// This will be passed to the callback provided to `Config::run(...)`. Functionality intended
/// to be executed on the render thread is exposed through this object.
pub struct RenderContext {
    __: u32, // This field is purely used to prevent API consumers from creating their own instance
}

impl RenderContext {
    #[inline]
    fn new() -> Self {
        RenderContext { __: 0 }
    }

    /// Finish the frame, syncing up with the main thread.
    #[inline]
    pub fn render_frame(&self) -> RenderFrame {
        unsafe { RenderFrame::from_u32(bgfx_sys::bgfx_render_frame()).unwrap() }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidContext,
    InvalidDisplay,
    InvalidWindow,
}

/// Bgfx configuration.
pub struct Config {
    context: *mut libc::c_void,
    device_id: u16,
    display: *mut libc::c_void,
    renderer: RendererType,
    vendor_id: u16,
    window: *mut libc::c_void,
}

impl Config {
    #[inline]
    fn new() -> Self {
        Config {
            context: ptr::null_mut(),
            device_id: 0,
            display: ptr::null_mut(),
            renderer: RendererType::Default,
            vendor_id: bgfx_sys::BGFX_PCI_ID_NONE,
            window: ptr::null_mut(),
        }
    }

    /// Sets the OpenGL context to use for rendering.
    #[inline]
    pub fn context(&mut self, context: *mut libc::c_void) -> &mut Self {
        self.context = context;
        self
    }

    /// Sets the desired device to use for rendering.
    #[inline]
    pub fn device(&mut self, vendor_id: Option<u16>, device_id: Option<u16>) -> &mut Self {
        self.device_id = device_id.unwrap_or(0);
        self.vendor_id = vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE);
        self
    }

    /// Sets the X11 display to render to.
    #[inline]
    pub fn display(&mut self, display: *mut libc::c_void) -> &mut Self {
        self.display = display;
        self
    }

    /// Sets the initial size of the render area.
    #[inline]
    pub fn renderer(&mut self, renderer: RendererType) -> &mut Self {
        self.renderer = renderer;
        self
    }

    /// Sets the window to render to.
    #[inline]
    pub fn window(&mut self, window: *mut libc::c_void) -> &mut Self {
        self.window = window;
        self
    }

    /// Verifies that all the required options have been passed.
    fn verify_opts(&self) -> Result<(), ConfigError> {
        if self.context == ptr::null_mut() {
            if cfg!(any(bsd, linux, mac_os)) {
                return Err(ConfigError::InvalidContext);
            }
        }

        if self.display == ptr::null_mut() {
            if cfg!(any(bsd, linux)) {
                return Err(ConfigError::InvalidDisplay);
            }
        }

        if self.window == ptr::null_mut() {
            return Err(ConfigError::InvalidWindow);
        }

        Ok(())
    }

    /// Fires off the main and render threads. The calling thread becomes the main thread, and a
    /// new thread is spawned to use as the main thread.
    pub fn run<M, R>(&self, main: M, render: R) -> Result<(), ConfigError>
        where M: Send + 'static + FnOnce(MainContext),
              R: FnOnce(RenderContext)
    {
        let result = self.verify_opts();
        if result.is_err() {
            return result;
        }

        // Set the bgfx platform data.
        unsafe {
            let mut data = bgfx_sys::Struct_bgfx_platform_data {
                ndt: self.display,
                nwh: self.window,
                context: self.context,
                backBuffer: ptr::null_mut(),
                backBufferDS: ptr::null_mut(),
            };

            bgfx_sys::bgfx_set_platform_data(&mut data);
        }

        // We need to assign the bgfx render thread *before* the main thread calls `init`. If not,
        // bgfx will spawn a new thread and assign that as the render thread.
        unsafe {
            bgfx_sys::bgfx_render_frame();
        }

        // Many platforms require rendering to happen in the actual main thread. As such, we need
        // to fire up a new thread to use as the application main thread.
        let renderer = self.renderer;
        let vendor = self.vendor_id;
        let device = self.device_id;
        let main_thread = thread::spawn(move || {
            unsafe {
                let success = bgfx_sys::bgfx_init(renderer as bgfx_sys::bgfx_renderer_type_t,
                                                  vendor,
                                                  device,
                                                  ptr::null_mut(),
                                                  ptr::null_mut());
                assert!(success != 0);
            }
            main(MainContext::new());
        });

        // Adopt the current thread as the render thread.
        render(RenderContext::new());

        // Pump the renderer until it has shut down properly.
        unsafe {
            while bgfx_sys::bgfx_render_frame() != bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT {
                thread::sleep_ms(1);
            }
        }

        main_thread.join().unwrap();

        Ok(())
    }
}

/// Creates a `Bgfx` object. Only one instance may exist at any given point in time.
#[inline]
pub fn create() -> Config {
    Config::new()
}

/// Creates a view matrix for looking at a point.
#[inline]
pub fn mtx_look_at(eye: &[f32; 3], at: &[f32; 3]) -> [f32; 16] {
    unsafe {
        let mut mat: [f32; 16] = mem::uninitialized();
        bgfx_sys::bx_mtx_look_at(mat.as_mut_ptr(), eye.as_ptr(), at.as_ptr(), ptr::null());
        mat
    }
}

/// Creates a projection matrix.
#[inline]
pub fn mtx_proj(fovy: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    unsafe {
        let mut mat: [f32; 16] = mem::uninitialized();
        bgfx_sys::bx_mtx_proj(mat.as_mut_ptr(), fovy, aspect, near, far, false);
        mat
    }
}

/// Creates a rotation matrix for rotating around both the X and Y axes.
#[inline]
pub fn mtx_rotate_xy(x: f32, y: f32) -> [f32; 16] {
    unsafe {
        let mut mat: [f32; 16] = mem::uninitialized();
        bgfx_sys::bx_mtx_rotate_xy(mat.as_mut_ptr(), x, y);
        mat
    }
}
