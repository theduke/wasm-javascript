# wasmtime_js

Run Javascript in the [wasmtime](https://github.com/bytecodealliance/wasmtime)
Webassembly runtime.

## Usage

See [the example](./examples/interop.rs).

**Performance Notice**: 
`wasmtime` is very slow in debug mode.

Compiling the spidermonkey engine takes quite a long time. 
Always run with `--release`, or set the opt-level for wasmtime crates in the debug profile.
