#!/bin/bash

export RUSTFLAGS="
    -C default-linker-libraries \
    -Z external-clangrt \
    -Z validate-mir \
    -Z verify-llvm-ir \
    -Z mir-opt-level=2 \
    -Z share-generics=yes \
    -Z remap-cwd-prefix=. \
    -Z function-sections=yes \
    -Z dep-info-omit-d-target \
    -C relocation-model=static \
    -C symbol-mangling-version=v0 \
    -C llvm-args=-fp-contract=off \
    -C llvm-args=-enable-misched \
    -C llvm-args=-enable-post-misched \
    -C llvm-args=-enable-dfa-jump-thread \
    -C link-args=-Wl,--sort-section=alignment \
    -C link-args=-Wl,-O3,--gc-sections,--as-needed \
    -C link-args=-Wl,-z,relro,-z,now,-x,-z,noexecstack,-s,--strip-all
" 

cargo update

export CARGO_TERM_COLOR=always

export JEMALLOC_SYS_DISABLE_WARN_ERROR=1

export CCACHE_DISABLE=1

cargo +nightly zigbuild --target "aarch64-unknown-linux-musl" --bin "pga_demo" -Z build-std -Z trim-paths
