# rusty-hermit

[![Build Status](https://git.rwth-aachen.de/acs/public/hermitcore/rusty-hermit/badges/master/pipeline.svg)](https://git.rwth-aachen.de/acs/public/hermitcore/rusty-hermit/pipelines)
![Actions Status](https://github.com/hermitcore/rusty-hermit/workflows/Test/badge.svg)
[![Slack Status](https://radiant-ridge-95061.herokuapp.com/badge.svg)](https://radiant-ridge-95061.herokuapp.com)

These are helper crates to build applications for the unikernel [RustyHermit](https://github.com/hermitcore/libhermit-rs).

Please read the README of [RustyHermit](https://github.com/hermitcore/libhermit-rs) for more information.

## Components:

- **demo:** Example application that executes a set of tests/benchmarks
- **hermit-abi:** Exposes `libhermit-rs`' interface, so that you can link against the static kernel library
- **hermit-sys:** Crate-level interface. Just include this crate in your `Cargo.toml` to make your application an unikernel-application. (The magic happens in the `build.rs` script.)
- **libhermit-rs:** The kernel itself.
- **loader:** Kernel loader to load the RustyHermit binary in Qemu, as Qemu cannot run 64-bit binaries directly but has to go through the x86 boot-process.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
