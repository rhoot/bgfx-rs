rust-bindgen:
	git clone https://github.com/servo/rust-bindgen.git

rust-bindgen/target/release/bindgen: rust-bindgen
	cd rust-bindgen && cargo build --release

src/ffi_bgfx.rs: rust-bindgen/target/release/bindgen bgfx/include/bgfx/c99/bgfx.h bgfx/include/bgfx/c99/platform.h
	rust-bindgen/target/release/bindgen \
		--constified-enum bgfx_renderer_type \
		--constified-enum bgfx_attrib \
		--constified-enum bgfx_render_frame \
		-o src/ffi_bgfx.rs \
		bgfx/include/bgfx/c99/platform.h \
		-- \
		-Ibgfx/include \
		-Ibx/include \
		-include bgfx/c99/bgfx.h
