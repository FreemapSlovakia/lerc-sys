# lerc-sys

[![Crates.io](https://img.shields.io/crates/v/lerc-sys)](https://crates.io/crates/lerc-sys)
[![Docs.rs](https://docs.rs/lerc-sys/badge.svg)](https://docs.rs/lerc-sys)

Low-level Rust FFI bindings to [Esri's LERC](https://github.com/Esri/lerc) compression library (C API).

This crate provides raw, unsafe bindings generated via `bindgen`, and builds the LERC C++ source using `cc`.

## Build

This crate **vendored** the LERC C++ sources and compiles them automatically using `cc`. It does not require a system-installed `libLerc`.

## Status

✅ Supports LERC 4.0+
✅ Linux tested
✅ Automatically generates bindings to `Lerc_c_api.h`

## License

Apache-2.0
