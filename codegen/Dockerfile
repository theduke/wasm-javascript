FROM archlinux

RUN pacman -Sy base-devel build-essential python3 python-pip wget rustup binaryen
RUN rustup toolchain install nightly && rustup default nightly && rustup target add wasm32-wasi

RUN cd /host && make
