# efi_ffi

[![Crates.io](https://img.shields.io/crates/v/efi)](https://crates.io/crates/efi_ffi)

A crate that provides raw declarations of UEFI functions, types and constants which can be used to build UEFI applications in Rust.

To learn how to use it, see the [efi](https://github.com/gurry/efi/tree/master) crate. For exmple, see how the UEFI declarations from this crate [are used in `efi`'s `Console` module to access UEFI console functionality](https://github.com/gurry/efi/blob/422d9762ec5857d42c8095c74aef14cae3cc2020/src/console.rs#L1-L40).

Currently only a subset of the UEFI API is implemented. Pull requests are welcome for parts that are missing.

The version of UEFI spec implemented by this crate is `2.4`.
