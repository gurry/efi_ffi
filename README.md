# efi_ffi

[![Crates.io](https://img.shields.io/crates/v/efi)](https://crates.io/crates/efi_ffi)

A crate that provides raw declarations of UEFI functions, types and constants which can be used to build UEFI applications in Rust.

To learn how to use it, see the [efi](https://github.com/gurry/efi/tree/master) crate. For exmple, see how the UEFI declarations from this crate [are used in `efi`'s `Console` module to access UEFI console functionality](https://github.com/gurry/efi/blob/master/src/console.rs).

Currently only a subset of the UEFI API is implemented. Pull requests are welcome for parts that are missing.
