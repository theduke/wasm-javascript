ROOT:=$(dir $(abspath $(firstword $(MAKEFILE_LIST))))

# Build the spidermonkey.wasm file.
spidermonkey:
	git submodule update --init
	make -C $(ROOT)witx-bindgen/crates/gen-spidermonkey/spidermonkey-wasm all
	mkdir -p $(ROOT)../wasmtime_js/wasm
	cp $(ROOT)witx-bindgen/crates/gen-spidermonkey/spidermonkey-wasm/spidermonkey.wasm $(ROOT)../wasmtime_js/wasm/spidermonkey.wasm

# Install required dependencies on Arch linux.
arch-setup:
	pacman -Sy --needed --noconfirm base-devel python3 python-pip wget rustup binaryen git
	rustup toolchain install nightly
	rustup default nightly
	rustup target add wasm32-wasi

wasm:
	cargo run
	# cat $(ROOT)../wasmtime_js/wasm/js.wasm | gzip -9 > $(ROOT)../wasmtime_js/wasm/js.wasm.gz
	# cat $(ROOT)../wasmtime_js/wasm/spidermonkey.wasm | gzip -9 > $(ROOT)../wasmtime_js/wasm/spidermonkey.wasm.gz

arch-spidermonkey: arch-setup spidermonkey

# Build spidermonkey.wasm in a docker container
docker:
	docker run --rm -v $(ROOT):/host archlinux bash -c "pacman -Sy --noconfirm make && cd /host/codegen && make arch-spidermonkey"
