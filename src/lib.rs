extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;

use std::marker::PhantomData;
use std::mem;
use std::option::Option;
use std::ptr;
use std::thread;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RendererType {
    Null = bgfx_sys::BGFX_RENDERER_TYPE_NULL,
    Direct3D9 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9,
    Direct3D11 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11,
    Direct3D12 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12,
    Metal = bgfx_sys::BGFX_RENDERER_TYPE_METAL,
    OpenGLES = bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES,
    OpenGL = bgfx_sys::BGFX_RENDERER_TYPE_OPENGL,
    Vulkan = bgfx_sys::BGFX_RENDERER_TYPE_VULKAN,
    Default = bgfx_sys::BGFX_RENDERER_TYPE_COUNT,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RenderFrame {
    NoContext = bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT,
    Render = bgfx_sys::BGFX_RENDER_FRAME_RENDER,
    Exiting = bgfx_sys::BGFX_RENDER_FRAME_EXITING,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Attrib {
    Position = bgfx_sys::BGFX_ATTRIB_POSITION,
    Normal = bgfx_sys::BGFX_ATTRIB_NORMAL,
    Tangent = bgfx_sys::BGFX_ATTRIB_TANGENT,
    Bitangent = bgfx_sys::BGFX_ATTRIB_BITANGENT,
    Color0 = bgfx_sys::BGFX_ATTRIB_COLOR0,
    Color1 = bgfx_sys::BGFX_ATTRIB_COLOR1,
    Indices = bgfx_sys::BGFX_ATTRIB_INDICES,
    Weight = bgfx_sys::BGFX_ATTRIB_WEIGHT,
    TexCoord0 = bgfx_sys::BGFX_ATTRIB_TEXCOORD0,
    TexCoord1 = bgfx_sys::BGFX_ATTRIB_TEXCOORD1,
    TexCoord2 = bgfx_sys::BGFX_ATTRIB_TEXCOORD2,
    TexCoord3 = bgfx_sys::BGFX_ATTRIB_TEXCOORD3,
    TexCoord4 = bgfx_sys::BGFX_ATTRIB_TEXCOORD4,
    TexCoord5 = bgfx_sys::BGFX_ATTRIB_TEXCOORD5,
    TexCoord6 = bgfx_sys::BGFX_ATTRIB_TEXCOORD6,
    TexCoord7 = bgfx_sys::BGFX_ATTRIB_TEXCOORD7,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AttribType {
    UInt8 = bgfx_sys::BGFX_ATTRIB_TYPE_UINT8,
    UInt10 = bgfx_sys::BGFX_ATTRIB_TYPE_UINT10,
    Int16 = bgfx_sys::BGFX_ATTRIB_TYPE_INT16,
    Half = bgfx_sys::BGFX_ATTRIB_TYPE_HALF,
    Float = bgfx_sys::BGFX_ATTRIB_TYPE_FLOAT,
}


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
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> Memory<'a> {
    /// Copies the source data into a new bgfx-managed buffer, and returns said buffer.
    ///
    /// IMPORTANT: If this buffer is never passed into a bgfx call, the memory will never be freed.
    ///
    /// # Arguments
    ///
    /// * `data` - Array of data to copy into this buffer.
    pub fn copy<'b, T>(data: &'b [T]) -> Memory<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_copy(data.as_ptr() as *const libc::c_void,
                                             mem::size_of_val(data) as u32);

            Memory { handle: handle, _phantom: PhantomData }
        }
    }

    /// Creates a reference to the source data for passing into bgfx, and returns said reference.
    /// When using this constructor over the `copy` call, no copy will be created. Bgfx will read
    /// the source memory directly.
    ///
    /// # Arguments
    ///
    /// * `data` - Array of data to create a reference to.
    pub fn reference<T>(data: &'a [T]) -> Memory<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_make_ref(data.as_ptr() as *const libc::c_void,
                                                 mem::size_of_val(data) as u32);

            Memory { handle: handle, _phantom: PhantomData }
        }
    }
}

/// Represents a shader program.
///
/// The program holds a vertex shader and a fragment shaders.
pub struct Program<'a> {
    handle: bgfx_sys::bgfx_program_handle_t,
    _vsh: Shader<'a>,
    _fsh: Shader<'a>,
}

impl<'a> Program<'a> {
    /// Creates a new program from a vertex shader and a fragment shader. Ownership is moved to the
    /// program.
    ///
    /// # Arguments
    ///
    /// * `vsh` - The vertex shader.
    /// * `fsh` - The fragment shader.
    pub fn new(vsh: Shader<'a>, fsh: Shader<'a>) -> Program<'a> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_program(vsh.handle, fsh.handle, 0);

            Program { handle: handle, _vsh: vsh, _fsh: fsh }
        }
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        unsafe {
            bgfx_sys::bgfx_destroy_program(self.handle);
        }
    }
}

/// Represents a shader.
pub struct Shader<'a> {
    handle: bgfx_sys::bgfx_shader_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> Shader<'a> {
    /// Creates a new shader from bgfx-managed memory.
    ///
    /// # Arguments
    ///
    /// * `context` - Reference to the main thread context.
    /// * `data` - Memory to create the shader from. Ownership is claimed.
    pub fn new(context: &'a MainContext, data: Memory<'a>) -> Shader<'a> {
        let _ = context;

        unsafe {
            let handle = bgfx_sys::bgfx_create_shader(data.handle);

            Shader { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            bgfx_sys::bgfx_destroy_shader(self.handle);
        }
    }
}

/// A buffer holding vertex indices, with each triplet defining a triangle.
pub struct IndexBuffer<'a> {
    handle: bgfx_sys::bgfx_index_buffer_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> IndexBuffer<'a> {
    /// Creates a new index buffer containing the given bgfx-managed memory.
    ///
    /// # Arguments
    ///
    /// * `context` - Reference to the main thread context.
    /// * `indices` - Indices to create the index buffer from.
    /// * `flags` - Index buffer creation flags.
    pub fn new(context: &'a MainContext,
               indices: Memory<'a>,
               flags: BufferFlags)
               -> IndexBuffer<'a> {
        let _ = context;

        unsafe {
            let handle = bgfx_sys::bgfx_create_index_buffer(indices.handle, flags.bits());

            IndexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for IndexBuffer<'a> {
    fn drop(&mut self) {
        unsafe {
            bgfx_sys::bgfx_destroy_index_buffer(self.handle);
        }
    }
}

/// A buffer holding vertices, each representing a point in space.
pub struct VertexBuffer<'a> {
    handle: bgfx_sys::bgfx_vertex_buffer_handle_t,
    _phantom: PhantomData<&'a MainContext>,
}

impl<'a> VertexBuffer<'a> {
    /// Creates a new vertex buffer containing the given bgfx-managed memory.
    ///
    /// # Arguments
    ///
    /// * `context` - Reference to the main thread context.
    /// * `verts` - Vertices to create the vertex buffer from.
    /// * `decl` - `VertexDecl` describing the structure of each vertex.
    /// * `flags` - Vertex buffer creation flags.
    pub fn new<'b>(context: &'a MainContext,
                   verts: Memory<'a>,
                   decl: &'b VertexDecl,
                   flags: BufferFlags)
                   -> VertexBuffer<'a> {
        let _ = context;

        unsafe {
            let handle = bgfx_sys::bgfx_create_vertex_buffer(verts.handle,
                                                             &decl.decl,
                                                             flags.bits());

            VertexBuffer { handle: handle, _phantom: PhantomData }
        }
    }
}

impl<'a> Drop for VertexBuffer<'a> {
    fn drop(&mut self) {
        unsafe {
            bgfx_sys::bgfx_destroy_vertex_buffer(self.handle);
        }
    }
}

/// Describes the structure of a vertex.
pub struct VertexDecl {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDecl {
    /// Creates a new vertex declaration using a builder.
    ///
    /// # Arguments
    ///
    /// * `renderer` - Optional renderer this vertex declaration applies to. If not specified,
    ///                `RendererType::Null` is assumed.
    ///
    /// # Example
    ///
    /// ```
    /// let decl = VertexDecl::new(None)
    ///          .add(bgfx::Attrib::Position, 3, bgfx::AttribType::Float, None, None)
    ///          .add(bgfx::Attrib::Color0, 4, bgfx::AttribType::UInt8, Some(true), None)
    ///          .end();
    /// ```
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
    ///
    /// # Arguments
    ///
    /// * `attrib` - The field's attribute.
    /// * `count` - Amount of values this field is built up of.
    /// * `kind` - Type of the field.
    /// * `normalized` - Optional normalization. When using a fixed point `kind`, the value will be
    ///                  normalized to be 0.0-1.0 in the vertex shader rather than the raw actual
    ///                  values. The default is `false`.
    /// * `as_int` - Optional packaging rule `AttribType::UInt8` and `AttribType::UInt16`. The
    ///              default is `false`.
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
    ///
    /// # Arguments
    ///
    /// * `count` - Size of the gap, in bytes.
    pub fn skip(&mut self, count: u8) -> &mut Self {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_skip(&mut self.decl, count);
        }

        self
    }

    /// Finalizes the construction of the `VertexDecl`.
    pub fn end(&mut self) -> VertexDecl {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_end(&mut self.decl);
        }

        VertexDecl { decl: self.decl }
    }
}

/// Main thread context.
///
/// This will be passed to the callback provided to `Application::run(...)`. Functionality intended
/// to be executed on the main thread is exposed through this object.
pub struct MainContext {
    did_init: bool,
}

impl MainContext {
    /// Initializes bgfx.
    ///
    /// This must be done before any other call. May only be called once.
    ///
    /// # Arguments
    ///
    /// * `renderer` - Optional type of renderer to use. The default is the most appropriate one
    ///                available on the system.
    /// * `vendor_id` - Optional vendor ID of the device to use. If `None` is provided, the first
    ///                 available device will be used.
    /// * `device_id` - Optional device ID of the graphics card to use. If `None` is provided, the
    ///                 first available device will be used.
    #[inline]
    pub fn init(&mut self,
                renderer: Option<RendererType>,
                vendor_id: Option<u16>,
                device_id: Option<u16>)
                -> bool {
        assert!(!self.did_init);

        unsafe {
            let res = bgfx_sys::bgfx_init(
                renderer.unwrap_or(RendererType::Default) as bgfx_sys::bgfx_renderer_type_t,
                vendor_id.unwrap_or(bgfx_sys::BGFX_PCI_ID_NONE),
                device_id.unwrap_or(0_u16),
                ptr::null_mut(),
                ptr::null_mut()
            );

            self.did_init = res != 0;
        }

        self.did_init
    }

    /// Resets the graphics device to the given size.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the back buffer, in pixels.
    /// * `height` - Height of the back buffer, in pixels.
    /// * `reset` - Flags for the device reset.
    #[inline]
    pub fn reset(&self, width: u16, height: u16, reset: ResetFlags) {
        unsafe {
            bgfx_sys::bgfx_reset(width as u32, height as u32, reset.bits());
        }
    }

    /// Sets the debug flags in use.
    ///
    /// # Arguments
    ///
    /// * `debug` - Debug flags to use.
    #[inline]
    pub fn set_debug(&self, debug: DebugFlags) {
        unsafe {
            bgfx_sys::bgfx_set_debug(debug.bits());
        }
    }

    /// Sets the options to use when clearing the given view.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the view to set properties for.
    /// * `flags` - Flags specifying how to clear.
    /// * `rgba` - Color to fill the color buffer with, in RGBA format.
    /// * `depth` - Value to fill the depth buffer with.
    /// * `stencil` - Value to fill the stencil buffer with.
    #[inline]
    pub fn set_view_clear(&self, id: u8, flags: ClearFlags, rgba: u32, depth: f32, stencil: u8) {
        unsafe {
            bgfx_sys::bgfx_set_view_clear(id, flags.bits(), rgba, depth, stencil);
        }
    }

    /// Sets the rectangle to display the given view in.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the view.
    /// * `x` - X coordinate of the view rectangle.
    /// * `y` - Y coordinate of the view rectangle.
    /// * `width` - Width of the view rectangle.
    /// * `height` - Height of the view rectangle.
    #[inline]
    pub fn set_view_rect(&self, id: u8, x: u16, y: u16, width: u16, height: u16) {
        unsafe {
            bgfx_sys::bgfx_set_view_rect(id, x, y, width, height);
        }
    }

    /// Touch the view. ( ͡° ͜ʖ ͡°)
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the view.
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

        unsafe {
            bgfx_sys::bgfx_dbg_text_clear(attr, small);
        }
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
                                          pitch);
        }
    }

    /// Displays text in the debug text overlay.
    #[inline]
    pub fn dbg_text_print(&self, x: u16, y: u16, attr: u8, text: &str) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr() as *const i8);
        }
    }

    /// Finish the frame, syncing up with the render thread. Returns an incrementing frame counter.
    #[inline]
    pub fn frame(&self) -> u32 {
        unsafe { bgfx_sys::bgfx_frame() }
    }
}

impl Drop for MainContext {
    #[inline]
    fn drop(&mut self) {
        if self.did_init {
            unsafe {
                bgfx_sys::bgfx_shutdown();
            }
        }
    }
}

/// Render thread context.
///
/// This will be passed to the callback provided to `Application::run(...)`. Functionality intended
/// to be executed on the render thread is exposed through this object.
pub struct RenderContext {
    __: u32, // This field is purely used to prevent API consumers from creating their own instance
}

impl RenderContext {
    /// Finish the frame, syncing up with the main thread.
    #[inline]
    pub fn render_frame(&self) -> RenderFrame {
        unsafe {
            let max = bgfx_sys::BGFX_RENDER_FRAME_COUNT;
            let res = bgfx_sys::bgfx_render_frame();
            assert!(res < max);

            mem::transmute(res)
        }
    }
}

/// Bgfx-based application object.
///
/// Acts as the entry point to your application. The application object is responsible for firing
/// off the threads, and syncing up with them as they shut down.
pub struct Application {
    __: u32, // This field is purely used to prevent consumers from creating their own instance
}

impl Application {
    /// Runs the application.
    ///
    /// # Arguments
    ///
    /// * `main` - Entry point for the main thread. It will be passed a single `&mut MainContext`
    ///            for interacting with the library.
    /// * `render` - Entry point for the render thread. It will be passed a single '&RenderContext'
    ///              for interacting with the library.
    pub fn run<M, R>(&self, main: M, render: R)
        where M: Send + 'static + FnOnce(&mut MainContext),
              R: FnOnce(&RenderContext)
    {
        // We need to launch the render thread *before* the main thread starts
        // executing things, so let's do it now.
        let ctx = RenderContext { __: 0 };
        ctx.render_frame();

        // Many platforms require rendering to happen on the main thread. With
        // *no* platform is this a problem. As such, we spawn a *new* thread
        // to use as the main thread, and adopt the current one as the render
        // thread.
        let main_thread = thread::spawn(move || {
            let mut ctx = MainContext { did_init: false };
            main(&mut ctx);
        });

        render(&ctx);
        while ctx.render_frame() != RenderFrame::NoContext {
            thread::sleep_ms(1);
        }

        main_thread.join().unwrap();
    }
}

/// Creates an `Application`. Only one instance may exist at any given point in time.
pub fn create(display: *mut libc::c_void,
              window: *mut libc::c_void,
              context: *mut libc::c_void)
              -> Application {
    // TODO: Only allow one instance

    let mut data = bgfx_sys::Struct_bgfx_platform_data {
        ndt: display,
        nwh: window,
        context: context,
        backBuffer: ptr::null_mut(),
        backBufferDS: ptr::null_mut(),
    };

    unsafe {
        bgfx_sys::bgfx_set_platform_data(&mut data);
    }

    Application { __: 0 }
}
