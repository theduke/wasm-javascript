# wasmtime_js

Run Javascript in the [wasmtime](https://github.com/bytecodealliance/wasmtime)
Webassembly runtime.

## Usage

Cargo.toml:
```toml
[dependencies]
wasmtime_js = { git = "https://github.com/theduke/wasm-javascript" }
```

See [examples/interop.js](./examples/interop.rs) for a code example.

**Performance Notice**: 
`wasmtime` is *very slow* in debug mode.
Compiling the spidermonkey engine takes quite a long time. 
Always run with `--release`, or set the opt-level for wasmtime crates in the 
debug profile.
