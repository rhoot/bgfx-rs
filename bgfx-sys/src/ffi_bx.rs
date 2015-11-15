// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern "C" {
    pub fn bx_mtx_look_at(result: *mut f32, eye: *const f32, at: *const f32, up: *const f32) -> ();
    pub fn bx_mtx_proj(result: *mut f32, fovy: f32, aspect: f32, near: f32, far: f32,
                       oglNdc: bool) -> ();
    pub fn bx_mtx_rotate_xy(result: *mut f32, ax: f32, ay: f32) -> ();
}
