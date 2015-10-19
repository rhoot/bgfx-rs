use std::marker::PhantomData;
use std::mem;
use std::option::Option;

use bgfx_sys;
use libc;

use super::RendererType;

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

pub struct VertexDeclBuilder {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

pub struct VertexDecl {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDeclBuilder {
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

    pub fn skip(&mut self, count: u8) -> &mut Self {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_skip(&mut self.decl, count);
        }

        self
    }

    pub fn end(&mut self) -> VertexDecl {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_end(&mut self.decl);
        }

        VertexDecl { decl: self.decl }
    }
}

impl VertexDecl {
    pub fn new(renderer: Option<RendererType>) -> VertexDeclBuilder {
        unsafe {
            let mut descr = VertexDeclBuilder {
                decl: mem::uninitialized::<bgfx_sys::Struct_bgfx_vertex_decl>(),
            };

            let renderer = renderer.unwrap_or(RendererType::Null) as bgfx_sys::bgfx_renderer_type_t;
            bgfx_sys::bgfx_vertex_decl_begin(&mut descr.decl, renderer);

            descr
        }
    }
}

pub struct VertexBuffer<'a> {
    handle: bgfx_sys::bgfx_vertex_buffer_handle_t,
    _phantom: PhantomData<&'a super::MainContext>,
}

impl<'a> VertexBuffer<'a> {
    pub fn new<'b, T>(_context: &'a super::MainContext,
                      verts: &'b [T],
                      decl: &'b VertexDecl,
                      flags: super::BufferFlags)
                      -> VertexBuffer<'a> {
        unsafe {
            let mem_ref = bgfx_sys::bgfx_make_ref(verts.as_ptr() as *const libc::c_void,
                                                  mem::size_of_val(verts) as u32);

            let handle = bgfx_sys::bgfx_create_vertex_buffer(mem_ref, &decl.decl, flags.bits());

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

pub struct IndexBuffer<'a> {
    handle: bgfx_sys::bgfx_index_buffer_handle_t,
    _phantom: PhantomData<&'a super::MainContext>,
}

impl<'a> IndexBuffer<'a> {
    pub fn new<'b, T>(_context: &'a super::MainContext,
                      indices: &'b [T],
                      flags: super::BufferFlags)
                      -> IndexBuffer<'a> {
        unsafe {
            let mem_ref = bgfx_sys::bgfx_make_ref(indices.as_ptr() as *const libc::c_void,
                                                  mem::size_of_val(indices) as u32);

            let handle = bgfx_sys::bgfx_create_index_buffer(mem_ref, flags.bits());

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
