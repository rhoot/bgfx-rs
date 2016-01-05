// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

#[macro_use]
extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate num;

use std::ffi;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use num::FromPrimitive;

mod flags;

pub use flags::*;

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

#[derive(Debug)]
pub enum BgfxError {
    InvalidDisplay,
    InvalidWindow,
    InitFailed,
}

/// A bgfx-managed buffer of memory.
///
/// It can be created by either copying existing data through `copy(...)`, or by referencing
/// existing memory directly through `reference(...)`.
pub struct Memory<'b> {
    handle: *const bgfx_sys::bgfx_memory_t,
    _phantom: PhantomData<&'b ()>,
}

impl<'b> Memory<'b> {
    /// Copies the source data into a new bgfx-managed buffer.
    ///
    /// IMPORTANT: If this buffer is never passed into a bgfx call, the memory will never be freed,
    /// and will leak.
    #[inline]
    pub fn copy<'d, T>(_bgfx: &'b Bgfx, data: &'d [T]) -> Memory<'b> {
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
    pub fn reference<T>(_bgfx: &'b Bgfx, data: &'b [T]) -> Memory<'b> {
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
pub struct Program<'s> {
    handle: bgfx_sys::bgfx_program_handle_t,
    _vsh: Shader<'s>,
    _fsh: Shader<'s>,
}

impl<'s> Program<'s> {
    /// Creates a new program from a vertex shader and a fragment shader. Ownerships of the shaders
    /// are moved to the program.
    #[inline]
    pub fn new(vsh: Shader<'s>, fsh: Shader<'s>) -> Program<'s> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_program(vsh.handle, fsh.handle, 0);
            Program { handle: handle, _vsh: vsh, _fsh: fsh }
        }
    }
}

impl<'s> Drop for Program<'s> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_program(self.handle) }
    }
}

/// Represents a shader.
pub struct Shader<'m> {
    handle: bgfx_sys::bgfx_shader_handle_t,
    _phantom: PhantomData<&'m ()>,
}

impl<'m> Shader<'m> {
    /// Creates a new shader from bgfx-managed memory.
    #[inline]
    pub fn new(data: Memory<'m>) -> Shader<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_shader(data.handle);
            Shader { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'m> Drop for Shader<'m> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_shader(self.handle) }
    }
}

/// A buffer holding vertex indices.
pub struct IndexBuffer<'m> {
    handle: bgfx_sys::bgfx_index_buffer_handle_t,
    _phantom: PhantomData<&'m Bgfx>,
}

impl<'m> IndexBuffer<'m> {
    /// Creates a new index buffer from bgfx-managed memory.
    #[inline]
    pub fn new(indices: Memory<'m>, flags: BufferFlags) -> IndexBuffer<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_index_buffer(indices.handle, flags.bits());
            IndexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'m> Drop for IndexBuffer<'m> {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_index_buffer(self.handle) }
    }
}

/// A buffer holding vertices, each representing a point in space.
pub struct VertexBuffer<'m> {
    handle: bgfx_sys::bgfx_vertex_buffer_handle_t,
    _phantom: PhantomData<&'m Bgfx>,
}

impl<'m> VertexBuffer<'m> {
    /// Creates a new vertex buffer from bgfx-managed memory.
    #[inline]
    pub fn new<'v>(verts: Memory<'m>,
                   decl: &'v VertexDecl,
                   flags: BufferFlags)
                   -> VertexBuffer<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_vertex_buffer(verts.handle,
                                                             &decl.decl,
                                                             flags.bits());
            VertexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'m> Drop for VertexBuffer<'m> {
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
    /// let decl = bgfx::VertexDecl::new(None)
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
pub struct Bgfx {
    _dummy: u32,
}

impl Bgfx {

    #[inline]
    fn new() -> Bgfx {
        Bgfx { _dummy: 0 }
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
        let text = ffi::CString::new(text).unwrap();
        unsafe { bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr()) }
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
    pub fn set_state(&self, state: StateFlags, rgba: Option<u32>) {
        unsafe { bgfx_sys::bgfx_set_state(state.bits(), rgba.unwrap_or(0)) }
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

impl Drop for Bgfx {
    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_shutdown() }
    }
}

#[inline]
pub fn render_frame() -> RenderFrame {
    unsafe { RenderFrame::from_u32(bgfx_sys::bgfx_render_frame()).unwrap() }
}

pub struct PlatformData {
    data: bgfx_sys::Struct_bgfx_platform_data,
}

impl PlatformData {

    #[inline]
    pub fn new() -> PlatformData {
        PlatformData {
            data: bgfx_sys::Struct_bgfx_platform_data {
                ndt: ptr::null_mut(),
                nwh: ptr::null_mut(),
                context: ptr::null_mut(),
                backBuffer: ptr::null_mut(),
                backBufferDS: ptr::null_mut(),
            },
        }
    }

    #[inline]
    pub fn context(&mut self, context: *mut libc::c_void) -> &mut Self {
        self.data.context = context;
        self
    }

    #[inline]
    pub fn display(&mut self, display: *mut libc::c_void) -> &mut Self {
        self.data.ndt = display;
        self
    }

    #[inline]
    pub fn window(&mut self, window: *mut libc::c_void) -> &mut Self {
        self.data.nwh = window;
        self
    }

    #[inline]
    pub fn apply(&mut self) -> Result<(), BgfxError> {
        if self.data.ndt == ptr::null_mut() && cfg!(target_os = "linux") {
            Err(BgfxError::InvalidDisplay)
        } else if self.data.nwh == ptr::null_mut() {
            Err(BgfxError::InvalidWindow)
        } else {
            unsafe {
                bgfx_sys::bgfx_set_platform_data(&mut self.data);
            }
            Ok(())
        }
    }
}

pub fn init(renderer: RendererType,
            vendor_id: Option<u16>,
            device_id: Option<u16>)
            -> Result<Bgfx, BgfxError> {
    let renderer = renderer as bgfx_sys::bgfx_renderer_type_t;
    let vendor = vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE);
    let device = device_id.unwrap_or(0);

    unsafe {
        let success = bgfx_sys::bgfx_init(renderer,
                                          vendor,
                                          device,
                                          ptr::null_mut(),
                                          ptr::null_mut());

        if success != 0 { Ok(Bgfx::new()) } else { Err(BgfxError::InitFailed) }
    }
}
