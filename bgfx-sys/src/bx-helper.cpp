#include <bx/fpumath.h>

extern "C" {

void bx_mtx_look_at(float* __restrict _result, float* __restrict _eye, float* __restrict _at, float* __restrict _up) {
    bx::mtxLookAt(_result, _eye, _at, _up);
}

void bx_mtx_proj(float* _result, float _fovy, float _aspect, float _near, float _far, bool _oglNdc) {
    bx::mtxProj(_result, _fovy, _aspect, _near, _far, _oglNdc);
}

void bx_mtx_rotate_xy(float* _result, float _ax, float _ay) {
    bx::mtxRotateXY(_result, _ax, _ay);
}

} // extern "C"
