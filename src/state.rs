// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx_sys;

pub struct State(u64);

impl State {
    pub fn new() -> State {
        State(bgfx_sys::BGFX_STATE_DEFAULT)
    }

    pub fn to_bits(&self) -> u64 {
        self.0
    }

// TODO: Flag setters
}
